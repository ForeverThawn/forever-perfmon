use std::mem::zeroed;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetLocalTime(system_time: *mut SystemTime);
}

pub fn current_time_display() -> String {
    let time = local_time();
    const MONTHS: [&str; 13] = [
        "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let month = MONTHS.get(time.month as usize).copied().unwrap_or("???");
    format!(
        "{month}. {:02}, {:04}   {:02}:{:02}:{:02}",
        time.day, time.year, time.hour, time.minute, time.second
    )
}

pub fn current_time_file() -> String {
    let time = local_time();
    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        time.year, time.month, time.day, time.hour, time.minute, time.second
    )
}

pub fn current_time_saved() -> String {
    let time = local_time();
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        time.year, time.month, time.day, time.hour, time.minute, time.second
    )
}

fn local_time() -> SystemTime {
    unsafe {
        let mut time: SystemTime = zeroed();
        GetLocalTime(&mut time);
        time
    }
}
