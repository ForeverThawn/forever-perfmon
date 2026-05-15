use std::fs;
use std::path::Path;

use crate::app::time::current_time_saved;

#[derive(Clone, Copy)]
pub struct Snapshot {
    pub timestamp: u64,
    pub disk_read_sum: f64,
    pub disk_write_sum: f64,
    pub network_received_sum: f64,
    pub network_sent_sum: f64,
    pub tailscale_received_absolute_bytes: Option<u64>,
    pub tailscale_sent_absolute_bytes: Option<u64>,
}

pub fn read_snapshot(path: &Path) -> Option<Snapshot> {
    let raw = fs::read_to_string(path).ok()?;
    Some(Snapshot {
        timestamp: json_u64(&raw, "timestamp").unwrap_or(0),
        disk_read_sum: json_f64(&raw, "disk_read_sum").unwrap_or(0.0),
        disk_write_sum: json_f64(&raw, "disk_write_sum").unwrap_or(0.0),
        network_received_sum: json_f64(&raw, "network_received_sum").unwrap_or(0.0),
        network_sent_sum: json_f64(&raw, "network_sent_sum").unwrap_or(0.0),
        tailscale_received_absolute_bytes: json_u64(&raw, "tailscale_received_absolute_bytes"),
        tailscale_sent_absolute_bytes: json_u64(&raw, "tailscale_sent_absolute_bytes"),
    })
}

pub fn write_snapshot(path: &Path, snapshot: &Snapshot) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let tailscale_received = snapshot
        .tailscale_received_absolute_bytes
        .map(|v| v.to_string())
        .unwrap_or_else(|| "null".to_string());
    let tailscale_sent = snapshot
        .tailscale_sent_absolute_bytes
        .map(|v| v.to_string())
        .unwrap_or_else(|| "null".to_string());
    let body = format!(
        concat!(
            "{{\n",
            "  \"version\": 5,\n",
            "  \"saved_at\": \"{}\",\n",
            "  \"timestamp\": {},\n",
            "  \"disk_read_sum\": {:.3},\n",
            "  \"disk_write_sum\": {:.3},\n",
            "  \"network_received_sum\": {:.3},\n",
            "  \"network_sent_sum\": {:.3},\n",
            "  \"tailscale_received_absolute_bytes\": {},\n",
            "  \"tailscale_sent_absolute_bytes\": {}\n",
            "}}\n"
        ),
        current_time_saved(),
        snapshot.timestamp,
        snapshot.disk_read_sum,
        snapshot.disk_write_sum,
        snapshot.network_received_sum,
        snapshot.network_sent_sum,
        tailscale_received,
        tailscale_sent
    );
    let _ = fs::write(path, body);
}

fn json_f64(raw: &str, key: &str) -> Option<f64> {
    json_token(raw, key)?.parse().ok()
}

fn json_u64(raw: &str, key: &str) -> Option<u64> {
    json_token(raw, key)?.parse().ok()
}

fn json_token(raw: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let start = raw.find(&needle)?;
    let after_colon = raw[start..].find(':')? + start + 1;
    let rest = raw[after_colon..].trim_start();
    if rest.starts_with("null") {
        return None;
    }
    let token: String = rest
        .chars()
        .take_while(|ch| ch.is_ascii_digit() || matches!(ch, '.' | '-' | '+' | 'e' | 'E'))
        .collect();
    (!token.is_empty()).then_some(token)
}
