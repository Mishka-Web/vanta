use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Backspace,
    Escape,
    Tab,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Left,
    Right,
    Up,
    Down,
    F(u8),
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyModifiers {
    bits: u8,
}

impl KeyModifiers {
    pub const SHIFT: u8 = 0b0001;
    pub const CTRL: u8 = 0b0010;
    pub const ALT: u8 = 0b0100;

    pub fn new(shift: bool, ctrl: bool, alt: bool) -> Self {
        let mut bits = 0;
        if shift { bits |= Self::SHIFT; }
        if ctrl { bits |= Self::CTRL; }
        if alt { bits |= Self::ALT; }
        Self { bits }
    }
    pub fn shift(self) -> bool { self.bits & Self::SHIFT != 0 }
    pub fn ctrl(self) -> bool { self.bits & Self::CTRL != 0 }
    pub fn alt(self) -> bool { self.bits & Self::ALT != 0 }
    pub fn is_empty(self) -> bool { self.bits == 0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub repeat: u16,
}

pub fn read_key() -> io::Result<KeyEvent> {
    use std::mem::MaybeUninit;
    let mut record = MaybeUninit::<InputRecord>::uninit();
    loop {
        let mut read = 0u32;
        let ok = unsafe { ReadConsoleInputW(GetStdHandle(STD_INPUT_HANDLE), record.as_mut_ptr(), 1, &mut read) };
        if ok == 0 { return Err(io::Error::last_os_error()); }
        let record = unsafe { record.assume_init() };
        if record.event_type != KEY_EVENT { continue; }
        let event = unsafe { record.event.key_event };
        if event.key_down == 0 || event.repeat_count == 0 { continue; }
        if is_modifier_key(event.virtual_key_code) { continue; }
        return Ok(decode_key_event(event));
    }
}

fn is_modifier_key(virtual_key_code: u16) -> bool {
    matches!(
        virtual_key_code,
        VK_SHIFT
            | VK_CONTROL
            | VK_MENU
            | VK_LSHIFT
            | VK_RSHIFT
            | VK_LCONTROL
            | VK_RCONTROL
            | VK_LMENU
            | VK_RMENU
    )
}

fn decode_key_event(event: KeyEventRecord) -> KeyEvent {
    let modifiers = decode_modifiers(event.control_key_state);
    let code = match event.virtual_key_code {
        VK_RETURN => KeyCode::Enter,
        VK_BACK => KeyCode::Backspace,
        VK_ESCAPE => KeyCode::Escape,
        VK_TAB => KeyCode::Tab,
        VK_DELETE => KeyCode::Delete,
        VK_INSERT => KeyCode::Insert,
        VK_HOME => KeyCode::Home,
        VK_END => KeyCode::End,
        VK_PRIOR => KeyCode::PageUp,
        VK_NEXT => KeyCode::PageDown,
        VK_LEFT => KeyCode::Left,
        VK_RIGHT => KeyCode::Right,
        VK_UP => KeyCode::Up,
        VK_DOWN => KeyCode::Down,
        VK_F1..=VK_F12 => KeyCode::F((event.virtual_key_code - VK_F1 + 1) as u8),
        _ if event.unicode_char != 0 => KeyCode::Char(char::from_u32(event.unicode_char as u32).unwrap_or('\0')),
        other => KeyCode::Unknown(other),
    };
    KeyEvent { code, modifiers, repeat: event.repeat_count }
}

fn decode_modifiers(state: u32) -> KeyModifiers {
    KeyModifiers::new(
        state & SHIFT_PRESSED != 0,
        state & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED) != 0,
        state & (LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED) != 0,
    )
}

const STD_INPUT_HANDLE: u32 = (-10i32) as u32;
const KEY_EVENT: u16 = 0x0001;
const VK_BACK: u16 = 0x0008;
const VK_SHIFT: u16 = 0x0010;
const VK_CONTROL: u16 = 0x0011;
const VK_MENU: u16 = 0x0012;
const VK_LSHIFT: u16 = 0x00A0;
const VK_RSHIFT: u16 = 0x00A1;
const VK_LCONTROL: u16 = 0x00A2;
const VK_RCONTROL: u16 = 0x00A3;
const VK_LMENU: u16 = 0x00A4;
const VK_RMENU: u16 = 0x00A5;
const VK_TAB: u16 = 0x0009;
const VK_RETURN: u16 = 0x000D;
const VK_ESCAPE: u16 = 0x001B;
const VK_PRIOR: u16 = 0x0021;
const VK_NEXT: u16 = 0x0022;
const VK_END: u16 = 0x0023;
const VK_HOME: u16 = 0x0024;
const VK_LEFT: u16 = 0x0025;
const VK_UP: u16 = 0x0026;
const VK_RIGHT: u16 = 0x0027;
const VK_DOWN: u16 = 0x0028;
const VK_INSERT: u16 = 0x002D;
const VK_DELETE: u16 = 0x002E;
const VK_F1: u16 = 0x0070;
const VK_F12: u16 = 0x007B;
const RIGHT_ALT_PRESSED: u32 = 0x0001;
const LEFT_ALT_PRESSED: u32 = 0x0002;
const RIGHT_CTRL_PRESSED: u32 = 0x0004;
const LEFT_CTRL_PRESSED: u32 = 0x0008;
const SHIFT_PRESSED: u32 = 0x0010;

#[repr(C)]
#[derive(Clone, Copy)]
struct InputRecord { event_type: u16, event: InputEvent }
#[repr(C)]
#[derive(Clone, Copy)]
union InputEvent { key_event: KeyEventRecord, raw: [u8; 16] }
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    fn ReadConsoleInputW(h_console_input: *mut std::ffi::c_void, lp_buffer: *mut InputRecord, n_length: u32, lp_number_of_events_read: *mut u32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modifiers_decode_independently() {
        let none = decode_modifiers(0); assert!(none.is_empty());
        let shift = decode_modifiers(SHIFT_PRESSED); assert!(shift.shift()); assert!(!shift.ctrl()); assert!(!shift.alt());
        assert!(decode_modifiers(LEFT_CTRL_PRESSED).ctrl());
        assert!(decode_modifiers(RIGHT_ALT_PRESSED).alt());
    }
    #[test]
    fn modifiers_can_be_combined() {
        let mods = decode_modifiers(SHIFT_PRESSED | LEFT_CTRL_PRESSED | RIGHT_ALT_PRESSED);
        assert!(mods.shift() && mods.ctrl() && mods.alt());
    }
    #[test]
    fn function_keys_map_to_number() {
        let event = KeyEventRecord { key_down: 1, repeat_count: 1, virtual_key_code: VK_F12, virtual_scan_code: 0, unicode_char: 0, control_key_state: 0 };
        assert_eq!(decode_key_event(event).code, KeyCode::F(12));
    }
    #[test]
    fn bare_modifier_keys_are_detected() {
        assert!(is_modifier_key(VK_SHIFT));
        assert!(is_modifier_key(VK_CONTROL));
        assert!(is_modifier_key(VK_MENU));
        assert!(is_modifier_key(VK_LSHIFT));
        assert!(is_modifier_key(VK_RCONTROL));
        assert!(!is_modifier_key(VK_TAB));
        assert!(!is_modifier_key(VK_F12));
    }

}
