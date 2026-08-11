use crate::screen::Screen;
use std::ffi::c_void;
use std::io::{self, Write};

pub struct Terminal {
    input_handle: *mut c_void,
    original_input_mode: u32,
    output_handle: *mut c_void,
    original_output_mode: u32,
}
impl Terminal {
    pub fn enter() -> io::Result<Self> {
        unsafe {
            let input_handle = GetStdHandle(STD_INPUT_HANDLE);
            let output_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if input_handle.is_null() || output_handle.is_null() {
                return Err(io::Error::last_os_error());
            }
            let mut input_mode = 0u32;
            let mut output_mode = 0u32;
            if GetConsoleMode(input_handle, &mut input_mode) == 0
                || GetConsoleMode(output_handle, &mut output_mode) == 0
            {
                return Err(io::Error::last_os_error());
            }
            let new_input_mode = input_mode
                & !(ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT)
                | ENABLE_WINDOW_INPUT;
            let new_output_mode = output_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            if SetConsoleMode(input_handle, new_input_mode) == 0 {
                return Err(io::Error::last_os_error());
            }
            if SetConsoleMode(output_handle, new_output_mode) == 0 {
                let _ = SetConsoleMode(input_handle, input_mode);
                return Err(io::Error::last_os_error());
            }
            let mut out = io::stdout();
            write!(out, "\x1b[?1049h\x1b[2J\x1b[H\x1b[?25h")?;
            out.flush()?;
            Ok(Self {
                input_handle,
                original_input_mode: input_mode,
                output_handle,
                original_output_mode: output_mode,
            })
        }
    }
}
impl Drop for Terminal {
    fn drop(&mut self) {
        unsafe {
            let _ = SetConsoleMode(self.input_handle, self.original_input_mode);
            let _ = SetConsoleMode(self.output_handle, self.original_output_mode);
        }
        let mut out = io::stdout();
        let _ = write!(out, "\x1b[?25h\x1b[?1049l");
        let _ = out.flush();
    }
}

pub fn terminal_size() -> io::Result<(u16, u16)> {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut info = std::mem::MaybeUninit::<ConsoleScreenBufferInfo>::uninit();
        if GetConsoleScreenBufferInfo(handle, info.as_mut_ptr()) == 0 {
            return Err(io::Error::last_os_error());
        }
        let info = info.assume_init();
        Ok((
            (info.window.right - info.window.left + 1).max(1) as u16,
            (info.window.bottom - info.window.top + 1).max(1) as u16,
        ))
    }
}
pub fn render(screen: &Screen) -> io::Result<()> {
    let mut out = io::stdout();
    let width = screen.width() as usize;
    write!(out, "\x1b[H")?;
    for (row_index, row) in screen.cells().chunks(width).enumerate() {
        for cell in row {
            write!(out, "{}", cell.ch())?;
        }
        if row_index + 1 < screen.height() as usize {
            write!(out, "\r\n")?;
        }
    }
    write!(
        out,
        "\x1b[{};{}H",
        screen.cursor_row() + 1,
        screen.cursor_col() + 1
    )?;
    out.flush()
}
const STD_INPUT_HANDLE: u32 = (-10i32) as u32;
const STD_OUTPUT_HANDLE: u32 = (-11i32) as u32;
const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
const ENABLE_LINE_INPUT: u32 = 0x0002;
const ENABLE_ECHO_INPUT: u32 = 0x0004;
const ENABLE_WINDOW_INPUT: u32 = 0x0008;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
#[repr(C)]
struct Coord {
    x: i16,
    y: i16,
}
#[repr(C)]
struct SmallRect {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}
#[repr(C)]
struct ConsoleScreenBufferInfo {
    size: Coord,
    cursor_position: Coord,
    attributes: u16,
    window: SmallRect,
    maximum_window_size: Coord,
}
unsafe extern "system" {
    fn GetStdHandle(n_std_handle: u32) -> *mut c_void;
    fn GetConsoleMode(h_console_handle: *mut c_void, lp_mode: *mut u32) -> i32;
    fn SetConsoleMode(h_console_handle: *mut c_void, dw_mode: u32) -> i32;
    fn GetConsoleScreenBufferInfo(
        h_console_output: *mut c_void,
        lp_console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
    ) -> i32;
}
