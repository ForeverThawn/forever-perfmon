use std::fs;
use std::io;
use std::path::PathBuf;

const CONFIG_FILE: &str = "forever-perfmon.toml";

pub struct Config {
    pub csv_dir: PathBuf,
    pub csv_output: bool,
    pub snapshot_file: PathBuf,
    pub hyperv_vm_name: Option<String>,
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let path = config_path();
        let raw = fs::read_to_string(&path)?;
        let csv_dir = required_string(&raw, "csv_dir")?;
        let csv_output = optional_bool(&raw, "csv_output").unwrap_or(true);
        let snapshot_dir = required_string(&raw, "snapshot_dir")?;
        let hyperv_vm_name = optional_string_or_false(&raw, "hyperv_vm_name")
            .unwrap_or_else(|| Some("ubuntu_22_04".to_string()));

        let csv_dir = PathBuf::from(csv_dir);
        let snapshot_dir = PathBuf::from(snapshot_dir);
        fs::create_dir_all(&csv_dir)?;
        fs::create_dir_all(&snapshot_dir)?;

        Ok(Self {
            csv_dir: fs::canonicalize(csv_dir)?,
            csv_output,
            snapshot_file: fs::canonicalize(snapshot_dir)?.join("snapshot.json"),
            hyperv_vm_name,
        })
    }
}

fn config_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(CONFIG_FILE)
}

fn required_string(raw: &str, key: &str) -> io::Result<String> {
    optional_string(raw, key).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("missing `{key}` in {CONFIG_FILE}"),
        )
    })
}

fn optional_string(raw: &str, key: &str) -> Option<String> {
    raw.lines()
        .find_map(|line| parse_string_assignment(line, key))
}

fn optional_bool(raw: &str, key: &str) -> Option<bool> {
    raw.lines()
        .find_map(|line| parse_bool_assignment(line, key))
}

fn optional_string_or_false(raw: &str, key: &str) -> Option<Option<String>> {
    if let Some(value) = optional_bool(raw, key) {
        return Some(value.then(|| "ubuntu_22_04".to_string()));
    }

    optional_string(raw, key).map(Some)
}

fn parse_bool_assignment(line: &str, key: &str) -> Option<bool> {
    let line = line.split_once('#').map_or(line, |(head, _)| head).trim();
    let (left, right) = line.split_once('=')?;
    if left.trim() != key {
        return None;
    }

    match right.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_string_assignment(line: &str, key: &str) -> Option<String> {
    let line = line.split_once('#').map_or(line, |(head, _)| head).trim();
    let (left, right) = line.split_once('=')?;
    if left.trim() != key {
        return None;
    }

    let value = right.trim();
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        return Some(unescape_basic_string(&value[1..value.len() - 1]));
    }
    (!value.is_empty()).then(|| value.to_string())
}

fn unescape_basic_string(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_toml_style_strings() {
        let raw = r#"
csv_dir = "X:\\_TEMP\\performancer_log"
csv_output = false
snapshot_dir = "D:\\snapshots"
hyperv_vm_name = "ubuntu_22_04"
"#;
        assert_eq!(
            optional_string(raw, "csv_dir").as_deref(),
            Some(r"X:\_TEMP\performancer_log")
        );
        assert_eq!(
            optional_string(raw, "hyperv_vm_name").as_deref(),
            Some("ubuntu_22_04")
        );
        assert_eq!(optional_bool(raw, "csv_output"), Some(false));
        assert_eq!(
            optional_string_or_false("hyperv_vm_name = false", "hyperv_vm_name"),
            Some(None)
        );
    }
}
