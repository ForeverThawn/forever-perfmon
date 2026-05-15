use std::ffi::OsStr;
use std::io;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};

use crate::app::format::round3;
use crate::app::types::{IfTotals, Sample};

const PDH_FMT_DOUBLE: u32 = 0x0000_0200;
const ERROR_SUCCESS: u32 = 0;
const IF_MAX_STRING_SIZE: usize = 256;
const IF_MAX_PHYS_ADDRESS_LENGTH: usize = 32;

#[repr(C)]
struct PdhFmtCounterValue {
    c_status: u32,
    double_value: f64,
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
    fn PdhCloseQuery(query: isize) -> u32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GlobalMemoryStatusEx(buffer: *mut MemoryStatusEx) -> i32;
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
    hyperv_avail: Option<isize>,
    hyperv_total: Option<isize>,
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
        let mut query = 0isize;
        let status = unsafe { PdhOpenQueryW(null(), 0, &mut query) };
        if status != ERROR_SUCCESS {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("PdhOpenQueryW failed: 0x{status:08x}"),
            ));
        }

        let counters = Self {
            query,
            cpu_usage: add_counter(query, r"\Processor(_Total)\% Processor Time")?,
            cpu_user_usage: add_counter(query, r"\Processor(_Total)\% User Time")?,
            cpu_privileged_usage: add_counter(query, r"\Processor(_Total)\% Privileged Time")?,
            cpu_dpc_usage: add_counter(query, r"\Processor(_Total)\% DPC Time")?,
            memory_committed_percentage: add_counter(query, r"\Memory\% Committed Bytes In Use")?,
            memory_avail: add_counter(query, r"\Memory\Available Bytes")?,
            memory_committed: add_counter(query, r"\Memory\Committed Bytes")?,
            memory_commit_limit: add_counter(query, r"\Memory\Commit Limit")?,
            disk_read: add_counter(query, r"\PhysicalDisk(_Total)\Disk Read Bytes/sec")?,
            disk_write: add_counter(query, r"\PhysicalDisk(_Total)\Disk Write Bytes/sec")?,
            hyperv_avail: hyperv_vm_name.and_then(|hyperv_vm_name| {
                add_counter(
                    query,
                    &format!(
                        r"\Hyper-V Dynamic Memory VM({})\guest available memory",
                        hyperv_vm_name
                    ),
                )
                .ok()
            }),
            hyperv_total: hyperv_vm_name.and_then(|hyperv_vm_name| {
                add_counter(
                    query,
                    &format!(
                        r"\Hyper-V Dynamic Memory VM({})\physical memory",
                        hyperv_vm_name
                    ),
                )
                .ok()
            }),
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

pub fn interface_totals() -> io::Result<IfTotals> {
    read_interface_table(|row| IfTotals {
        received_bytes: row.in_octets,
        sent_bytes: row.out_octets,
    })
    .map(|items| items.into_iter().fold(IfTotals::default(), add_totals))
}

pub fn tailscale_totals() -> io::Result<Option<IfTotals>> {
    let totals = read_interface_table(|row| {
        let alias = wide_array_to_string(&row.alias).to_ascii_lowercase();
        let description = wide_array_to_string(&row.description).to_ascii_lowercase();
        if alias.starts_with("tailscale") || description.contains("tailscale") {
            Some(IfTotals {
                received_bytes: row.in_octets,
                sent_bytes: row.out_octets,
            })
        } else {
            None
        }
    })?;

    let mut found = false;
    let mut sum = IfTotals::default();
    for totals in totals.into_iter().flatten() {
        found = true;
        sum = add_totals(sum, totals);
    }
    Ok(found.then_some(sum))
}

fn read_interface_table<T>(map: impl Fn(MibIfRow2) -> T) -> io::Result<Vec<T>> {
    let mut table_ptr: *mut MibIfTable2 = null_mut();
    let status = unsafe { GetIfTable2(&mut table_ptr) };
    if status != ERROR_SUCCESS {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("GetIfTable2 failed: 0x{status:08x}"),
        ));
    }

    let mut rows = Vec::new();
    unsafe {
        let count = (*table_ptr).num_entries as usize;
        let first = (*table_ptr).table.as_ptr();
        for i in 0..count {
            rows.push(map(*first.add(i)));
        }
        FreeMibTable(table_ptr.cast());
    }
    Ok(rows)
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

fn add_totals(left: IfTotals, right: IfTotals) -> IfTotals {
    IfTotals {
        received_bytes: left.received_bytes.saturating_add(right.received_bytes),
        sent_bytes: left.sent_bytes.saturating_add(right.sent_bytes),
    }
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
        let totals = interface_totals().expect("interface table should be readable");
        let _ = totals.received_bytes;
        let _ = totals.sent_bytes;
    }

    #[test]
    fn collects_pdh_sample() {
        let counters = PerfCounters::open(Some("ubuntu_22_04")).expect("pdh query should open");
        thread::sleep(Duration::from_secs(1));
        let sample = counters.collect().expect("pdh sample should collect");
        assert!(sample.cpu_usage >= 0.0);
    }
}
