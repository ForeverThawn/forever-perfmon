use std::ffi::OsStr;
use std::io;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use std::slice;

use crate::app::format::round3;
use crate::app::types::{IfTotals, Sample};

const PDH_FMT_DOUBLE: u32 = 0x0000_0200;
const ERROR_SUCCESS: u32 = 0;
const PDH_MORE_DATA: u32 = 0x8000_07d2;
const IF_MAX_STRING_SIZE: usize = 256;
const IF_MAX_PHYS_ADDRESS_LENGTH: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
struct PdhFmtCounterValue {
    c_status: u32,
    double_value: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PdhFmtCounterValueItemW {
    sz_name: *mut u16,
    fmt_value: PdhFmtCounterValue,
}

#[repr(C)]
struct MemoryStatusEx {
    dw_length: u32,
    dw_memory_load: u32,
    ull_total_phys: u64,
    ull_avail_phys: u64,
    ull_total_page_file: u64,
    ull_avail_page_file: u64,
    ull_total_virtual: u64,
    ull_avail_virtual: u64,
    ull_avail_extended_virtual: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MibIfRow2 {
    interface_luid: u64,
    interface_index: u32,
    interface_guid: Guid,
    alias: [u16; IF_MAX_STRING_SIZE + 1],
    description: [u16; IF_MAX_STRING_SIZE + 1],
    physical_address_length: u32,
    physical_address: [u8; IF_MAX_PHYS_ADDRESS_LENGTH],
    permanent_physical_address: [u8; IF_MAX_PHYS_ADDRESS_LENGTH],
    mtu: u32,
    if_type: u32,
    tunnel_type: u32,
    media_type: u32,
    physical_medium_type: u32,
    access_type: u32,
    direction_type: u32,
    interface_and_oper_status_flags: u8,
    oper_status: u32,
    admin_status: u32,
    media_connect_state: u32,
    network_guid: Guid,
    connection_type: u32,
    transmit_link_speed: u64,
    receive_link_speed: u64,
    in_octets: u64,
    in_ucast_pkts: u64,
    in_nucast_pkts: u64,
    in_discards: u64,
    in_errors: u64,
    in_unknown_protos: u64,
    in_ucast_octets: u64,
    in_multicast_octets: u64,
    in_broadcast_octets: u64,
    out_octets: u64,
    out_ucast_pkts: u64,
    out_nucast_pkts: u64,
    out_discards: u64,
    out_errors: u64,
    out_ucast_octets: u64,
    out_multicast_octets: u64,
    out_broadcast_octets: u64,
    out_qlen: u64,
}

#[repr(C)]
struct MibIfTable2 {
    num_entries: u32,
    table: [MibIfRow2; 1],
}

#[link(name = "pdh")]
unsafe extern "system" {
    fn PdhOpenQueryW(data_source: *const u16, user_data: usize, query: *mut isize) -> u32;
    fn PdhAddEnglishCounterW(
        query: isize,
        full_counter_path: *const u16,
        user_data: usize,
        counter: *mut isize,
    ) -> u32;
    fn PdhCollectQueryData(query: isize) -> u32;
    fn PdhGetFormattedCounterValue(
        counter: isize,
        format: u32,
        value_type: *mut u32,
        value: *mut PdhFmtCounterValue,
    ) -> u32;
    fn PdhGetFormattedCounterArrayW(
        counter: isize,
        format: u32,
        buffer_size: *mut u32,
        item_count: *mut u32,
        item_buffer: *mut PdhFmtCounterValueItemW,
    ) -> u32;
    fn PdhCloseQuery(query: isize) -> u32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GlobalMemoryStatusEx(buffer: *mut MemoryStatusEx) -> i32;
    fn GetPhysicallyInstalledSystemMemory(total_memory_in_kilobytes: *mut u64) -> i32;
}

#[link(name = "iphlpapi")]
unsafe extern "system" {
    fn GetIfTable2(table: *mut *mut MibIfTable2) -> u32;
    fn FreeMibTable(memory: *mut std::ffi::c_void);
}

pub struct PerfCounters {
    query: isize,
    cpu_usage: isize,
    cpu_user_usage: isize,
    cpu_privileged_usage: isize,
    cpu_dpc_usage: isize,
    memory_committed_percentage: isize,
    memory_avail: isize,
    memory_committed: isize,
    memory_commit_limit: isize,
    disk_read: isize,
    disk_write: isize,
    network_received: isize,
    network_sent: isize,
    hyperv_avail: Option<isize>,
    hyperv_total: Option<isize>,
}

pub struct NetworkTotals {
    pub all: IfTotals,
    pub tailscale: Option<IfTotals>,
}

struct PdhQuery {
    handle: isize,
}

struct MibTable {
    ptr: *mut MibIfTable2,
}

impl MibTable {
    fn get() -> io::Result<Self> {
        let mut ptr: *mut MibIfTable2 = null_mut();
        let status = unsafe { GetIfTable2(&mut ptr) };
        if status == ERROR_SUCCESS && !ptr.is_null() {
            Ok(Self { ptr })
        } else if status == ERROR_SUCCESS {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "GetIfTable2 returned a null table",
            ))
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("GetIfTable2 failed: 0x{status:08x}"),
            ))
        }
    }

    fn for_each_row(&self, mut f: impl FnMut(MibIfRow2)) {
        unsafe {
            let count = (*self.ptr).num_entries as usize;
            let first = (*self.ptr).table.as_ptr();
            for i in 0..count {
                f(*first.add(i));
            }
        }
    }
}

impl Drop for MibTable {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                FreeMibTable(self.ptr.cast());
            }
        }
    }
}

impl PdhQuery {
    fn open() -> io::Result<Self> {
        let mut handle = 0isize;
        let status = unsafe { PdhOpenQueryW(null(), 0, &mut handle) };
        if status == ERROR_SUCCESS {
            Ok(Self { handle })
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("PdhOpenQueryW failed: 0x{status:08x}"),
            ))
        }
    }

    fn into_raw(mut self) -> isize {
        let handle = self.handle;
        self.handle = 0;
        handle
    }
}

impl Drop for PdhQuery {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe {
                PdhCloseQuery(self.handle);
            }
        }
    }
}

impl Drop for PerfCounters {
    fn drop(&mut self) {
        if self.query != 0 {
            unsafe {
                PdhCloseQuery(self.query);
            }
        }
    }
}

impl PerfCounters {
    pub fn open(hyperv_vm_name: Option<&str>) -> io::Result<Self> {
        let query_guard = PdhQuery::open()?;
        let query = query_guard.handle;

        let cpu_usage = add_counter(query, r"\Processor(_Total)\% Processor Time")?;
        let cpu_user_usage = add_counter(query, r"\Processor(_Total)\% User Time")?;
        let cpu_privileged_usage = add_counter(query, r"\Processor(_Total)\% Privileged Time")?;
        let cpu_dpc_usage = add_counter(query, r"\Processor(_Total)\% DPC Time")?;
        let memory_committed_percentage = add_counter(query, r"\Memory\% Committed Bytes In Use")?;
        let memory_avail = add_counter(query, r"\Memory\Available Bytes")?;
        let memory_committed = add_counter(query, r"\Memory\Committed Bytes")?;
        let memory_commit_limit = add_counter(query, r"\Memory\Commit Limit")?;
        let disk_read = add_counter(query, r"\PhysicalDisk(_Total)\Disk Read Bytes/sec")?;
        let disk_write = add_counter(query, r"\PhysicalDisk(_Total)\Disk Write Bytes/sec")?;
        let network_received = add_counter(query, r"\Network Interface(*)\Bytes Received/sec")?;
        let network_sent = add_counter(query, r"\Network Interface(*)\Bytes Sent/sec")?;
        let hyperv_avail = hyperv_vm_name.and_then(|hyperv_vm_name| {
            add_counter(
                query,
                &format!(
                    r"\Hyper-V Dynamic Memory VM({})\guest available memory",
                    hyperv_vm_name
                ),
            )
            .ok()
        });
        let hyperv_total = hyperv_vm_name.and_then(|hyperv_vm_name| {
            add_counter(
                query,
                &format!(
                    r"\Hyper-V Dynamic Memory VM({})\physical memory",
                    hyperv_vm_name
                ),
            )
            .ok()
        });

        let counters = Self {
            query: query_guard.into_raw(),
            cpu_usage,
            cpu_user_usage,
            cpu_privileged_usage,
            cpu_dpc_usage,
            memory_committed_percentage,
            memory_avail,
            memory_committed,
            memory_commit_limit,
            disk_read,
            disk_write,
            network_received,
            network_sent,
            hyperv_avail,
            hyperv_total,
        };

        let status = unsafe { PdhCollectQueryData(counters.query) };
        if status != ERROR_SUCCESS {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("PdhCollectQueryData failed: 0x{status:08x}"),
            ));
        }

        Ok(counters)
    }

    pub fn collect(&self) -> io::Result<Sample> {
        let status = unsafe { PdhCollectQueryData(self.query) };
        if status != ERROR_SUCCESS {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("PdhCollectQueryData failed: 0x{status:08x}"),
            ));
        }

        Ok(Sample {
            cpu_usage: counter_value(self.cpu_usage).unwrap_or(0.0),
            cpu_user_usage: counter_value(self.cpu_user_usage).unwrap_or(0.0),
            cpu_privileged_usage: counter_value(self.cpu_privileged_usage).unwrap_or(0.0),
            cpu_dpc_usage: counter_value(self.cpu_dpc_usage).unwrap_or(0.0),
            memory_committed_percentage: counter_value(self.memory_committed_percentage)
                .unwrap_or(0.0),
            memory_avail: counter_value(self.memory_avail).unwrap_or(0.0),
            memory_committed: counter_value(self.memory_committed).unwrap_or(0.0),
            memory_commit_limit: counter_value(self.memory_commit_limit).unwrap_or(0.0),
            disk_read: counter_value(self.disk_read).unwrap_or(0.0),
            disk_write: counter_value(self.disk_write).unwrap_or(0.0),
            network_received: counter_array_sum(self.network_received).unwrap_or(0.0),
            network_sent: counter_array_sum(self.network_sent).unwrap_or(0.0),
            hyperv_avail_bytes: self
                .hyperv_avail
                .and_then(counter_value)
                .map(|mb| mb * 1_048_576.0),
            hyperv_total_bytes: self
                .hyperv_total
                .and_then(counter_value)
                .map(|mb| mb * 1_048_576.0),
        })
    }
}

pub fn physical_memory_total() -> u64 {
    if let Some(installed_memory) = physically_installed_memory_total() {
        return installed_memory;
    }

    unsafe {
        let mut status: MemoryStatusEx = zeroed();
        status.dw_length = size_of::<MemoryStatusEx>() as u32;
        if GlobalMemoryStatusEx(&mut status) != 0 {
            status.ull_total_phys
        } else {
            0
        }
    }
}

fn physically_installed_memory_total() -> Option<u64> {
    let mut total_kb = 0u64;
    let ok = unsafe { GetPhysicallyInstalledSystemMemory(&mut total_kb) != 0 };
    ok.then(|| total_kb.saturating_mul(1024))
        .filter(|total| *total > 0)
}

pub fn network_totals() -> io::Result<NetworkTotals> {
    let table = MibTable::get()?;
    let mut all = IfTotals::default();
    let mut tailscale = IfTotals::default();
    let mut tailscale_found = false;

    table.for_each_row(|row| {
        all = add_totals(
            all,
            IfTotals {
                received_bytes: row.in_octets,
                sent_bytes: row.out_octets,
            },
        );

        let alias = wide_array_to_string(&row.alias);
        let description = wide_array_to_string(&row.description);
        if is_tailscale_adapter(&alias, &description) {
            tailscale_found = true;
            tailscale = add_totals(
                tailscale,
                IfTotals {
                    received_bytes: row.in_octets,
                    sent_bytes: row.out_octets,
                },
            );
        }
    });

    Ok(NetworkTotals {
        all,
        tailscale: tailscale_found.then_some(tailscale),
    })
}

fn add_counter(query: isize, path: &str) -> io::Result<isize> {
    let wide = to_wide(path);
    let mut counter = 0isize;
    let status = unsafe { PdhAddEnglishCounterW(query, wide.as_ptr(), 0, &mut counter) };
    if status == ERROR_SUCCESS {
        Ok(counter)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("PdhAddEnglishCounterW failed for {path}: 0x{status:08x}"),
        ))
    }
}

fn counter_value(counter: isize) -> Option<f64> {
    let mut value_type = 0u32;
    let mut value = PdhFmtCounterValue {
        c_status: 0,
        double_value: 0.0,
    };
    let status = unsafe {
        PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, &mut value_type, &mut value)
    };
    if status == ERROR_SUCCESS && value.c_status == ERROR_SUCCESS {
        Some(round3(value.double_value))
    } else {
        None
    }
}

fn counter_array_sum(counter: isize) -> Option<f64> {
    let mut buffer_size = 0u32;
    let mut item_count = 0u32;
    let status = unsafe {
        PdhGetFormattedCounterArrayW(
            counter,
            PDH_FMT_DOUBLE,
            &mut buffer_size,
            &mut item_count,
            null_mut(),
        )
    };

    if status != PDH_MORE_DATA {
        return if status == ERROR_SUCCESS {
            Some(0.0)
        } else {
            None
        };
    }

    let item_size = size_of::<PdhFmtCounterValueItemW>();
    let item_slots = (buffer_size as usize)
        .div_ceil(item_size)
        .max(item_count as usize);
    let mut buffer = vec![
        PdhFmtCounterValueItemW {
            sz_name: null_mut(),
            fmt_value: PdhFmtCounterValue {
                c_status: 0,
                double_value: 0.0,
            },
        };
        item_slots
    ];
    buffer_size = (buffer.len() * item_size) as u32;

    let status = unsafe {
        PdhGetFormattedCounterArrayW(
            counter,
            PDH_FMT_DOUBLE,
            &mut buffer_size,
            &mut item_count,
            buffer.as_mut_ptr(),
        )
    };
    if status != ERROR_SUCCESS {
        return None;
    }

    let values = unsafe { slice::from_raw_parts(buffer.as_ptr(), item_count as usize) };
    Some(round3(
        values
            .iter()
            .filter(|item| item.fmt_value.c_status == ERROR_SUCCESS)
            .map(|item| item.fmt_value.double_value)
            .sum(),
    ))
}

fn add_totals(left: IfTotals, right: IfTotals) -> IfTotals {
    IfTotals {
        received_bytes: left.received_bytes.saturating_add(right.received_bytes),
        sent_bytes: left.sent_bytes.saturating_add(right.sent_bytes),
    }
}

fn is_tailscale_adapter(alias: &str, description: &str) -> bool {
    let alias = alias.to_ascii_lowercase();
    let description = description.to_ascii_lowercase();
    alias.starts_with("tailscale")
        && !alias.contains("npcap packet driver")
        && !description.contains("npcap packet driver")
}

fn to_wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}

fn wide_array_to_string(value: &[u16]) -> String {
    let len = value.iter().position(|ch| *ch == 0).unwrap_or(value.len());
    String::from_utf16_lossy(&value[..len])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn reads_windows_interface_totals() {
        let totals = network_totals()
            .expect("interface table should be readable")
            .all;
        let _ = totals.received_bytes;
        let _ = totals.sent_bytes;
    }

    #[test]
    #[ignore]
    fn prints_tailscale_interface_totals() {
        let totals = network_totals().expect("interface table should be readable");
        if let Some(tailscale) = totals.tailscale {
            println!(
                "tailscale received={} sent={}",
                tailscale.received_bytes, tailscale.sent_bytes
            );
        } else {
            println!("tailscale not found");
        }
    }

    #[test]
    #[ignore]
    fn prints_tailscale_interface_rows() {
        let table = MibTable::get().expect("interface table should be readable");
        table.for_each_row(|row| {
            let alias = wide_array_to_string(&row.alias);
            if alias.to_ascii_lowercase().starts_with("tailscale") {
                let description = wide_array_to_string(&row.description);
                println!(
                    "index={} alias={alias} description={description} in={} out={}",
                    row.interface_index, row.in_octets, row.out_octets
                );
            }
        });
    }

    #[test]
    fn collects_pdh_sample() {
        let counters = PerfCounters::open(Some("ubuntu_22_04")).expect("pdh query should open");
        thread::sleep(Duration::from_secs(1));
        let sample = counters.collect().expect("pdh sample should collect");
        assert!(sample.cpu_usage >= 0.0);
        assert!(sample.network_received.is_finite() && sample.network_received >= 0.0);
        assert!(sample.network_sent.is_finite() && sample.network_sent >= 0.0);
    }
}
