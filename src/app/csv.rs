use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::app::format::{format_bytes, percentage};
use crate::app::time::current_time_file;
use crate::app::types::Sample;

pub struct CsvWriter {
    writer: BufWriter<File>,
}

pub struct CsvRow {
    pub current_time: String,
    pub record_time: String,
    pub sample: Sample,
    pub memory_usage_percentage: f64,
    pub memory_used: f64,
    pub memory_total: f64,
    pub disk_read_sum: f64,
    pub disk_write_sum: f64,
    pub network_received_value: f64,
    pub network_sent_value: f64,
    pub network_received_sum: f64,
    pub network_sent_sum: f64,
    pub tailscale_received_latest: Option<u64>,
    pub tailscale_sent_latest: Option<u64>,
    pub tailscale_received_absolute: Option<u64>,
    pub tailscale_sent_absolute: Option<u64>,
}

impl CsvWriter {
    pub fn create(csv_dir: &Path) -> io::Result<Self> {
        fs::create_dir_all(csv_dir)?;
        let path = csv_dir.join(format!("performancer_{}.csv", current_time_file()));
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writeln!(
            writer,
            "Forever Performance Monitor 5,Record Time,CPU Usage,CPU User,CPU Privileged,CPU Driver,Physical Memory Usage,Physical Memory,Max Memory Size,Committed Usage,Committed Size,Max Committed Size,Disk Read Rate,Disk Write Rate,Network Receiving Rate,Network Sending Rate,Tailscale Received Rate,Tailscale Sent Rate,Total Disk Read,Total Disk Write,Total Network Received,Total Network Sent,Total Tailscale Received,Total Tailscale Sent,Hyper-V Memory Usage,Hyper-V Memory Available,Hyper-V Memory Total,"
        )?;
        Ok(Self { writer })
    }

    pub fn write_row(&mut self, row: &CsvRow) -> io::Result<()> {
        let (hyperv_usage_percentage, hyperv_usage, hyperv_total) =
            match (row.sample.hyperv_avail_bytes, row.sample.hyperv_total_bytes) {
                (Some(avail), Some(total)) if total > 0.0 => {
                    let usage = (total - avail).max(0.0);
                    (
                        format!("{:.3}", percentage(usage, total)),
                        format_bytes(usage),
                        format_bytes(total),
                    )
                }
                _ => (String::new(), String::new(), String::new()),
            };

        let fields = vec![
            row.current_time.clone(),
            row.record_time.clone(),
            format!("{:.3}", row.sample.cpu_usage),
            format!("{:.3}", row.sample.cpu_user_usage),
            format!("{:.3}", row.sample.cpu_privileged_usage),
            format!("{:.3}", row.sample.cpu_dpc_usage),
            format!("{:.3}", row.memory_usage_percentage),
            format_bytes(row.memory_used),
            format_bytes(row.memory_total),
            format!("{:.3}", row.sample.memory_committed_percentage),
            format_bytes(row.sample.memory_committed),
            format_bytes(row.sample.memory_commit_limit),
            format_bytes(row.sample.disk_read),
            format_bytes(row.sample.disk_write),
            format_bytes(row.network_received_value),
            format_bytes(row.network_sent_value),
            row.tailscale_received_latest
                .map(|v| format_bytes(v as f64))
                .unwrap_or_default(),
            row.tailscale_sent_latest
                .map(|v| format_bytes(v as f64))
                .unwrap_or_default(),
            format_bytes(row.disk_read_sum),
            format_bytes(row.disk_write_sum),
            format_bytes(row.network_received_sum),
            format_bytes(row.network_sent_sum),
            row.tailscale_received_absolute
                .map(|v| format_bytes(v as f64))
                .unwrap_or_default(),
            row.tailscale_sent_absolute
                .map(|v| format_bytes(v as f64))
                .unwrap_or_default(),
            hyperv_usage_percentage,
            hyperv_usage,
            hyperv_total,
        ];

        writeln!(
            self.writer,
            "{}",
            fields
                .iter()
                .map(|field| csv_escape(field))
                .collect::<Vec<_>>()
                .join(",")
        )?;
        self.writer.flush()
    }
}

fn csv_escape(field: &str) -> String {
    format!("\"{}\"", field.replace('"', "\"\""))
}
