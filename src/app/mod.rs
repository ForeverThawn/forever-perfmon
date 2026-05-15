mod config;
mod console;
mod csv;
mod format;
mod render;
mod snapshot;
mod tailscale;
mod time;
mod types;
mod windows;

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use config::Config;
use console::{Key, read_key, wait_for_resume_choice};
use csv::{CsvRow, CsvWriter};
use format::{format_elapsed, percentage};
use render::render_screen;
use snapshot::{Snapshot, read_snapshot, write_snapshot};
use tailscale::TailscaleTotalState;
use windows::{PerfCounters, interface_totals, physical_memory_total, tailscale_totals};

pub fn run() -> io::Result<()> {
    console::enable_ansi_colors();

    let config = Config::load()?;
    let snapshot = read_snapshot(&config.snapshot_file);
    let resume = choose_resume_mode(snapshot.as_ref());
    let hyperv_enabled = config.hyperv_vm_name.is_some();
    let counters = PerfCounters::open(config.hyperv_vm_name.as_deref())?;
    let memory_total = physical_memory_total();
    let mut csv = if config.csv_output {
        Some(CsvWriter::create(&config.csv_dir)?)
    } else {
        None
    };

    let mut timestamp = 0u64;
    let mut disk_read_sum = 0.0;
    let mut disk_write_sum = 0.0;
    let mut network_received_sum = 0.0;
    let mut network_sent_sum = 0.0;
    let mut tailscale_received_latest_state = TailscaleTotalState::new();
    let mut tailscale_sent_latest_state = TailscaleTotalState::new();
    let mut tailscale_received_delta_state = TailscaleTotalState::new();
    let mut tailscale_sent_delta_state = TailscaleTotalState::new();
    let mut tailscale_received_has_snapshot_baseline = false;
    let mut tailscale_sent_has_snapshot_baseline = false;

    if resume {
        if let Some(snapshot) = snapshot {
            timestamp = snapshot.timestamp;
            disk_read_sum = snapshot.disk_read_sum;
            disk_write_sum = snapshot.disk_write_sum;
            network_received_sum = snapshot.network_received_sum;
            network_sent_sum = snapshot.network_sent_sum;

            tailscale_received_has_snapshot_baseline =
                snapshot.tailscale_received_absolute_bytes.is_some();
            tailscale_sent_has_snapshot_baseline = snapshot.tailscale_sent_absolute_bytes.is_some();
            tailscale_received_latest_state =
                TailscaleTotalState::from_absolute(snapshot.tailscale_received_absolute_bytes);
            tailscale_sent_latest_state =
                TailscaleTotalState::from_absolute(snapshot.tailscale_sent_absolute_bytes);
        }
    }

    let mut previous_if_totals = interface_totals().unwrap_or_default();
    let mut previous_if_instant = Instant::now();
    let mut hyperv_total_old = None;

    clear_screen()?;

    loop {
        thread::sleep(Duration::from_secs(1));
        let elapsed = previous_if_instant.elapsed().as_secs_f64().max(0.001);
        previous_if_instant = Instant::now();

        let sample = match counters.collect() {
            Ok(sample) => sample,
            Err(_) => continue,
        };

        let current_if_totals = interface_totals().unwrap_or(previous_if_totals);
        let tailscale_absolute = tailscale_totals().ok().flatten();
        let network_received_value = current_if_totals
            .received_bytes
            .saturating_sub(previous_if_totals.received_bytes)
            as f64
            / elapsed;
        let network_sent_value = current_if_totals
            .sent_bytes
            .saturating_sub(previous_if_totals.sent_bytes) as f64
            / elapsed;
        previous_if_totals = current_if_totals;

        let memory_used = (memory_total as f64 - sample.memory_avail).max(0.0);
        let memory_usage_percentage = percentage(memory_used, memory_total as f64);
        disk_read_sum += sample.disk_read;
        disk_write_sum += sample.disk_write;
        network_received_sum += network_received_value;
        network_sent_sum += network_sent_value;

        let tailscale_received_absolute = tailscale_absolute.map(|totals| totals.received_bytes);
        let tailscale_sent_absolute = tailscale_absolute.map(|totals| totals.sent_bytes);
        let tailscale_received_delta = tailscale_received_delta_state
            .update(tailscale_received_absolute)
            .delta_bytes;
        let tailscale_sent_delta = tailscale_sent_delta_state
            .update(tailscale_sent_absolute)
            .delta_bytes;
        let tailscale_received_latest = if resume && tailscale_received_has_snapshot_baseline {
            tailscale_received_latest_state
                .update(tailscale_received_absolute)
                .latest_bytes
        } else {
            tailscale_received_absolute
        };
        let tailscale_sent_latest = if resume && tailscale_sent_has_snapshot_baseline {
            tailscale_sent_latest_state
                .update(tailscale_sent_absolute)
                .latest_bytes
        } else {
            tailscale_sent_absolute
        };

        let hyperv_allocating = matches!((hyperv_total_old, sample.hyperv_total_bytes), (Some(old), Some(new)) if old != new);
        if sample.hyperv_total_bytes.is_some() {
            hyperv_total_old = sample.hyperv_total_bytes;
        }

        let record_time = format_elapsed(timestamp);
        render_screen(
            &sample,
            memory_total,
            memory_used,
            memory_usage_percentage,
            disk_read_sum,
            disk_write_sum,
            network_received_value,
            network_sent_value,
            network_received_sum,
            network_sent_sum,
            tailscale_received_delta,
            tailscale_sent_delta,
            tailscale_received_latest,
            tailscale_sent_latest,
            tailscale_received_absolute,
            tailscale_sent_absolute,
            hyperv_enabled,
            hyperv_allocating,
            &record_time,
        )?;

        if let Some(csv) = csv.as_mut() {
            csv.write_row(&CsvRow {
                current_time: time::current_time_display(),
                record_time,
                sample,
                memory_usage_percentage,
                memory_used,
                memory_total: memory_total as f64,
                disk_read_sum,
                disk_write_sum,
                network_received_value,
                network_sent_value,
                network_received_sum,
                network_sent_sum,
                tailscale_received_latest,
                tailscale_sent_latest,
                tailscale_received_absolute,
                tailscale_sent_absolute,
            })?;
        }

        timestamp += 1;
        write_snapshot(
            &config.snapshot_file,
            &Snapshot {
                timestamp,
                disk_read_sum,
                disk_write_sum,
                network_received_sum,
                network_sent_sum,
                tailscale_received_absolute_bytes: tailscale_received_absolute,
                tailscale_sent_absolute_bytes: tailscale_sent_absolute,
            },
        );

        if let Some(key) = read_key() {
            match key {
                Key::Q => break,
                Key::C => clear_screen()?,
                Key::R => {}
            }
        }
    }

    Ok(())
}

fn choose_resume_mode(snapshot: Option<&Snapshot>) -> bool {
    if snapshot.is_none() {
        return false;
    }

    println!("{}", format::color("Snapshot found.", "32"));
    println!(
        "{}",
        format::color(
            "Press C to continue counters, or R to reset counters.",
            "33"
        )
    );
    wait_for_resume_choice()
}

fn clear_screen() -> io::Result<()> {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush()
}
