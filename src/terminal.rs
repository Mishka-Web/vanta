use std::io::{self, Write};

pub struct Terminal;

impl Terminal {
    pub fn enter() -> io::Result<Self> {
        enable_raw_input()?;
        Ok(Self)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_input();
        let mut out = io::stdout();
        let _ = show_cursor(&mut out);
        let _ = out.flush();
    }
}

pub fn clear_screen(out: &mut impl Write) -> io::Result<()> {
    write!(out, "\x1b[2J\x1b[H")
}

pub fn move_cursor(out: &mut impl Write, row: u16, col: u16) -> io::Result<()> {
    write!(out, "\x1b[{};{}H", row + 1, col + 1)
}

pub fn hide_cursor(out: &mut impl Write) -> io::Result<()> {
    write!(out, "\x1b[?25l")
}

pub fn show_cursor(out: &mut impl Write) -> io::Result<()> {
    write!(out, "\x1b[?25h")
}

#[cfg(windows)]
fn enable_raw_input() -> io::Result<()> {
    use std::ffi::c_void;

    const STD_INPUT_HANDLE: u32 = (-10i32) as u32;
    const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
    const ENABLE_LINE_INPUT: u32 = 0x0002;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;
    const ENABLE_WINDOW_INPUT: u32 = 0x0008;

    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        let mut mode = 0u32;

        if GetConsoleMode(handle, &mut mode) == 0 {
            return Err(io::Error::last_os_error());
        }

        let new_mode = mode
            & !(ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT)
            | ENABLE_WINDOW_INPUT;

        if SetConsoleMode(handle, new_mode) == 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(windows)]
fn disable_raw_input() -> io::Result<()> {
    Ok(())
}

#[cfg(windows)]
use std::ffi::c_void;

#[cfg(windows)]
extern "system" {
    fn GetStdHandle(n_std_handle: u32) -> *mut c_void;
    fn GetConsoleMode(h_console_handle: *mut c_void, lp_mode: *mut u32) -> i32;
    fn SetConsoleMode(h_console_handle: *mut c_void, dw_mode: u32) -> i32;
}
