pub fn format_elapsed(timestamp: u64) -> String {
    let days = timestamp / 86_400;
    let hours = (timestamp % 86_400) / 3_600;
    let minutes = (timestamp % 3_600) / 60;
    let seconds = timestamp % 60;
    format!("{days:>4}:{hours:>3}:{minutes:>3}:{seconds:>3}")
}

pub fn format_bytes(byte_size: f64) -> String {
    let abs = byte_size.abs();
    if abs < 1024.0 {
        format!("{byte_size:>15.0}  B ")
    } else if abs < 0x100000 as f64 {
        format!("{:>15.3} KB ", byte_size / 0x400 as f64)
    } else if abs < 0x40000000 as f64 {
        format!("{:>15.3} MB ", byte_size / 0x100000 as f64)
    } else if abs < 0x10000000000u64 as f64 {
        format!("{:>15.3} GB ", byte_size / 0x40000000 as f64)
    } else if abs < 0x4000000000000u64 as f64 {
        format!("{:>15.3} TB ", byte_size / 0x10000000000u64 as f64)
    } else {
        format!("{:>15.3} PB ", byte_size / 0x4000000000000u64 as f64)
    }
}

pub fn grouped_decimal(value: f64) -> String {
    let rounded = format!("{value:.3}");
    let (int_part, frac_part) = rounded.split_once('.').unwrap_or((&rounded, "000"));
    format!("{}.{}", grouped_digits(int_part), frac_part)
}

pub fn grouped_int(value: f64) -> String {
    grouped_digits(&format!("{:.0}", value.max(0.0)))
}

pub fn percentage(value: f64, total: f64) -> f64 {
    if total > 0.0 {
        value / total * 100.0
    } else {
        0.0
    }
}

pub fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

pub fn color(text: &str, code: &str) -> String {
    format!("\x1b[{code}m{text}\x1b[0m")
}

pub fn color_bg(text: &str, fg: &str, bg: &str) -> String {
    format!("\x1b[{fg};{bg}m{text}\x1b[0m")
}

fn grouped_digits(raw: &str) -> String {
    let (negative, digits) = raw
        .strip_prefix('-')
        .map(|rest| (true, rest))
        .unwrap_or((false, raw));
    let mut out = String::new();
    for (i, ch) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let mut grouped: String = out.chars().rev().collect();
    if negative {
        grouped.insert(0, '-');
    }
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_elapsed_time_like_the_script() {
        assert_eq!(format_elapsed(0), "   0:  0:  0:  0");
        assert_eq!(format_elapsed(90_061), "   1:  1:  1:  1");
        assert_eq!(format_elapsed(60 * 86_400), "  60:  0:  0:  0");
    }

    #[test]
    fn formats_bytes_with_binary_units() {
        assert_eq!(format_bytes(512.0), "            512  B ");
        assert_eq!(format_bytes(1536.0), "          1.500 KB ");
        assert_eq!(format_bytes(1_572_864.0), "          1.500 MB ");
        assert_eq!(
            format_bytes(10.0 * 0x10000000000u64 as f64),
            "         10.000 TB "
        );
    }
}
