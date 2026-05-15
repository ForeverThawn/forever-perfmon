use std::thread;
use std::time::Duration;

const STD_INPUT_HANDLE: u32 = 0xffff_fff6;
const STD_OUTPUT_HANDLE: u32 = 0xffff_fff5;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
const KEY_EVENT: u16 = 0x0001;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    C,
    Q,
    R,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct KeyEventRecord {
    key_down: i32,
    repeat_count: u16,
    virtual_key_code: u16,
    virtual_scan_code: u16,
    unicode_char: u16,
    control_key_state: u32,
}

#[repr(C)]
union InputEvent {
    key_event: KeyEventRecord,
    _padding: [u8; 16],
}

#[repr(C)]
struct InputRecord {
    event_type: u16,
    event: InputEvent,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetStdHandle(std_handle: u32) -> isize;
    fn GetConsoleMode(console_handle: isize, mode: *mut u32) -> i32;
    fn SetConsoleMode(console_handle: isize, mode: u32) -> i32;
    fn GetNumberOfConsoleInputEvents(console_input: isize, number_of_events: *mut u32) -> i32;
    fn ReadConsoleInputW(
        console_input: isize,
        buffer: *mut InputRecord,
        length: u32,
        number_of_events_read: *mut u32,
    ) -> i32;
}

pub fn enable_ansi_colors() {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == 0 || handle == -1 {
            return;
        }
        let mut mode = 0u32;
        if GetConsoleMode(handle, &mut mode) != 0 {
            let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

pub fn wait_for_resume_choice() -> bool {
    loop {
        match read_key() {
            Some(Key::C) => return true,
            Some(Key::R) => return false,
            _ => thread::sleep(Duration::from_millis(25)),
        }
    }
}

pub fn read_key() -> Option<Key> {
    let input = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    if input == 0 || input == -1 {
        return None;
    }

    loop {
        let mut event_count = 0u32;
        if unsafe { GetNumberOfConsoleInputEvents(input, &mut event_count) } == 0
            || event_count == 0
        {
            return None;
        }

        let mut record = InputRecord {
            event_type: 0,
            event: InputEvent { _padding: [0; 16] },
        };
        let mut events_read = 0u32;
        if unsafe { ReadConsoleInputW(input, &mut record, 1, &mut events_read) } == 0
            || events_read == 0
        {
            return None;
        }

        if record.event_type != KEY_EVENT {
            continue;
        }

        let key = unsafe { record.event.key_event };
        if key.key_down == 0 {
            continue;
        }

        match char::from_u32(key.unicode_char as u32).map(|ch| ch.to_ascii_lowercase()) {
            Some('c') => return Some(Key::C),
            Some('q') => return Some(Key::Q),
            Some('r') => return Some(Key::R),
            _ => continue,
        }
    }
}
