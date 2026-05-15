use std::io::{self, Write};

use crate::app::format::{color, color_bg, format_bytes, grouped_decimal, grouped_int, percentage};
use crate::app::time::current_time_display;
use crate::app::types::Sample;

pub fn render_screen(
    sample: &Sample,
    memory_total: u64,
    memory_used: f64,
    memory_usage_percentage: f64,
    disk_read_sum: f64,
    disk_write_sum: f64,
    network_received_value: f64,
    network_sent_value: f64,
    network_received_sum: f64,
    network_sent_sum: f64,
    tailscale_received_delta: Option<u64>,
    tailscale_sent_delta: Option<u64>,
    tailscale_received_latest: Option<u64>,
    tailscale_sent_latest: Option<u64>,
    tailscale_received_absolute: Option<u64>,
    tailscale_sent_absolute: Option<u64>,
    hyperv_enabled: bool,
    hyperv_allocating: bool,
    record_time: &str,
) -> io::Result<()> {
    print!("\x1b[H");
    println!("{}", color("Forever Performance Monitor 5", "33"));
    print!("{}", color(&current_time_display(), "32"));
    print!("                ");
    println!("{}", color(record_time, "36"));

    print!("{}", color("CPU      Usage   : ", "33"));
    print_percentage(sample.cpu_usage, 15);
    print!("    {}", color_bg("[User]", "30", "100"));
    print_percentage(sample.cpu_user_usage, 7);
    print!("    {}", color_bg("[Privileged]", "30", "100"));
    print_percentage(sample.cpu_privileged_usage, 7);
    print!("    {}", color_bg("[Driver]", "30", "100"));
    print_percentage(sample.cpu_dpc_usage, 7);
    println!();

    print!("{}", color("Physical Memory  : ", "33"));
    print_percentage(memory_usage_percentage, 15);
    print!(
        "  {}",
        color(&format!("{:>21} B ", grouped_int(memory_used)), "90")
    );
    println!(
        "{}/{}",
        format_bytes(memory_used),
        format_bytes(memory_total as f64)
    );

    print!("{}", color("Total    Commited: ", "33"));
    print_percentage(sample.memory_committed_percentage, 15);
    print!(
        "  {}",
        color(
            &format!("{:>21} B ", grouped_int(sample.memory_committed)),
            "90"
        )
    );
    println!(
        "{}/{}",
        format_bytes(sample.memory_committed),
        format_bytes(sample.memory_commit_limit)
    );

    print_rate_line("Disk     Read    : ", sample.disk_read, disk_read_sum);
    print_rate_line("Disk     Write   : ", sample.disk_write, disk_write_sum);
    print_rate_line(
        "Network  Received: ",
        network_received_value,
        network_received_sum,
    );
    print_rate_line("Network  Sent    : ", network_sent_value, network_sent_sum);

    if hyperv_enabled {
        render_hyperv(sample, hyperv_allocating);
    }
    render_tailscale_line(
        "Tailscale Received:",
        tailscale_received_delta,
        tailscale_received_latest,
        tailscale_received_absolute,
    );
    render_tailscale_line(
        "Tailscale Sent    :",
        tailscale_sent_delta,
        tailscale_sent_latest,
        tailscale_sent_absolute,
    );
    io::stdout().flush()
}

fn print_rate_line(label: &str, rate: f64, total: f64) {
    print!("{}", color(label, "33"));
    print!("{}", format_bytes(rate));
    print!(
        "{} ",
        color(&format!("{:>21} B", grouped_decimal(rate)), "90")
    );
    println!(
        "{}{}",
        color(&format_bytes(total), "36"),
        color("Total", "36")
    );
}

fn render_hyperv(sample: &Sample, hyperv_allocating: bool) {
    print!("{}", color("Hyper-V  Memory  : ", "33"));
    match (sample.hyperv_avail_bytes, sample.hyperv_total_bytes) {
        (Some(avail), Some(total)) if total > 0.0 => {
            let usage = (total - avail).max(0.0);
            print_percentage(percentage(usage, total), 15);
            if hyperv_allocating {
                print!("{}", color("       [ALLOCATING]       ", "32"));
            } else {
                print!("                          ");
            }
            println!("{}/{}", format_bytes(usage), format_bytes(total));
        }
        _ => {
            println!("{}{}", " ".repeat(15), color("  unavailable", "90"));
        }
    }
}

fn render_tailscale_line(
    label: &str,
    delta: Option<u64>,
    latest: Option<u64>,
    absolute: Option<u64>,
) {
    print!("{}", color(label, "33"));
    let delta = delta
        .map(|v| format_bytes(v as f64))
        .unwrap_or_else(|| format!("{:>19}", ""));
    let latest = latest
        .map(|v| format!("{:>17} Latest", format_bytes(v as f64).trim()))
        .unwrap_or_else(|| format!("{:>24}", ""));
    let total = absolute
        .map(|v| format_bytes(v as f64))
        .unwrap_or_else(|| format!("{:>19}", ""));

    print!("{}", color(&delta, "37"));
    print!("{}", color(&latest, "36"));
    println!("{}{}", color(&total, "32"), color("Total", "32"));
}

fn print_percentage(value: f64, width: usize) {
    let text = format!("{value:>width$.3}  %");
    print!("{}", color(&text, percentage_color(value)));
}

fn percentage_color(value: f64) -> &'static str {
    if value < 0.0 {
        "37"
    } else if value < 12.5 {
        "36"
    } else if value < 25.0 {
        "96"
    } else if value < 37.5 {
        "32"
    } else if value < 50.0 {
        "92"
    } else if value < 62.5 {
        "33"
    } else if value < 75.0 {
        "93"
    } else if value < 87.5 {
        "35"
    } else if value < 100.0 {
        "31"
    } else {
        "91"
    }
}
