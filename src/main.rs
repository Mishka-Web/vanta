#![cfg_attr(not(windows), allow(dead_code))]

#[cfg(windows)]
mod input;
#[cfg(windows)]
mod terminal;

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    use std::io::{self, Write};

    let _terminal = terminal::Terminal::enter()?;
    let mut out = io::stdout();

    terminal::clear_screen(&mut out)?;
    terminal::hide_cursor(&mut out)?;
    terminal::move_cursor(&mut out, 0, 0)?;

    writeln!(out, "VANTA v0.0.1")?;
    writeln!(out)?;
    writeln!(out, "Keyboard-first editor runtime")?;
    writeln!(out)?;
    writeln!(out, "Press any key.")?;
    writeln!(out, "Press q to quit.")?;
    out.flush()?;

    loop {
        let key = input::read_key()?;

        terminal::clear_screen(&mut out)?;
        terminal::move_cursor(&mut out, 0, 0)?;

        writeln!(out, "VANTA v0.0.1")?;
        writeln!(out)?;
        writeln!(out, "KEY EVENT")?;
        writeln!(out, "---------")?;
        writeln!(out, "kind: {:?}", key)?;

        if matches!(key, input::Key::Char('q')) {
            break;
        }

        out.flush()?;
    }

    terminal::show_cursor(&mut out)?;
    terminal::clear_screen(&mut out)?;
    terminal::move_cursor(&mut out, 0, 0)?;
    out.flush()?;

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    eprintln!("VANTA v0.0.1 currently targets Windows only.");
}
