use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Unknown(u16),
}

pub fn read_key() -> io::Result<Key> {
    use std::mem::MaybeUninit;

    let mut record = MaybeUninit::<InputRecord>::uninit();

    loop {
        let mut read = 0u32;

        let ok = unsafe {
            ReadConsoleInputW(
                GetStdHandle(STD_INPUT_HANDLE),
                record.as_mut_ptr(),
                1,
                &mut read,
            )
        };

        if ok == 0 {
            return Err(io::Error::last_os_error());
        }

        let record = unsafe { record.assume_init() };

        if record.event_type != KEY_EVENT {
            continue;
        }

        let event = unsafe { record.event.key_event };

        if event.key_down == 0 || event.repeat_count == 0 {
            continue;
        }

        return Ok(match event.virtual_key_code {
            VK_RETURN => Key::Enter,
            VK_BACK => Key::Backspace,
            VK_ESCAPE => Key::Escape,
            VK_LEFT => Key::Left,
            VK_RIGHT => Key::Right,
            VK_UP => Key::Up,
            VK_DOWN => Key::Down,
            _ if event.unicode_char != 0 => {
                Key::Char(char::from_u32(event.unicode_char as u32).unwrap_or('\0'))
            }
            other => Key::Unknown(other),
        });
    }
}

const STD_INPUT_HANDLE: u32 = (-10i32) as u32;
const KEY_EVENT: u16 = 0x0001;

const VK_RETURN: u16 = 0x000D;
const VK_BACK: u16 = 0x0008;
const VK_ESCAPE: u16 = 0x001B;
const VK_LEFT: u16 = 0x0025;
const VK_UP: u16 = 0x0026;
const VK_RIGHT: u16 = 0x0027;
const VK_DOWN: u16 = 0x0028;

#[repr(C)]
#[derive(Clone, Copy)]
struct InputRecord {
    event_type: u16,
    event: InputEvent,
}

#[repr(C)]
#[derive(Clone, Copy)]
union InputEvent {
    key_event: KeyEventRecord,
    raw: [u8; 16],
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

unsafe extern "system" {
    fn GetStdHandle(n_std_handle: u32) -> *mut std::ffi::c_void;

    fn ReadConsoleInputW(
        h_console_input: *mut std::ffi::c_void,
        lp_buffer: *mut InputRecord,
        n_length: u32,
        lp_number_of_events_read: *mut u32,
    ) -> i32;
}
