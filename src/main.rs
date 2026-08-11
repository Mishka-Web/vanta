#![cfg_attr(not(windows), allow(dead_code))]

#[cfg(windows)]
mod input;
#[cfg(windows)]
mod screen;
#[cfg(windows)]
mod terminal;

#[cfg(windows)]
use input::Key;
#[cfg(windows)]
use screen::Screen;
#[cfg(windows)]
use std::io;

#[cfg(windows)]
fn main() -> io::Result<()> {
    let _terminal = terminal::Terminal::enter()?;
    let (width, height) = terminal::terminal_size()?;
    let mut screen = Screen::new(width, height);

    draw_ui(&mut screen, None);
    terminal::render(&screen)?;

    loop {
        let key = input::read_key()?;

        match key {
            Key::Char('q') => break,
            Key::Left => screen.move_cursor_left(),
            Key::Right => screen.move_cursor_right(),
            Key::Up => screen.move_cursor_up(),
            Key::Down => screen.move_cursor_down(),
            _ => {}
        }

        let (new_width, new_height) = terminal::terminal_size()?;
        if new_width != screen.width() || new_height != screen.height() {
            screen.resize(new_width, new_height);
        }

        draw_ui(&mut screen, Some(key));
        terminal::render(&screen)?;
    }

    Ok(())
}

#[cfg(windows)]
fn draw_ui(screen: &mut Screen, last_key: Option<Key>) {
    screen.clear();

    screen.write_str(0, 0, "VANTA v0.0.2");
    screen.write_str(2, 0, "Screen Engine");
    screen.write_str(4, 0, "Arrows move the virtual cursor.");
    screen.write_str(5, 0, "Press q to quit.");

    if let Some(key) = last_key {
        screen.write_str(7, 0, &format!("Last key: {:?}", key));
    }

    let status = format!(
        "size={}x{}  cursor={},{}",
        screen.width(),
        screen.height(),
        screen.cursor_col(),
        screen.cursor_row()
    );

    let row = screen.height().saturating_sub(1);
    screen.write_str(row, 0, &status);
}

#[cfg(not(windows))]
fn main() {
    eprintln!("VANTA v0.0.2 currently targets Windows only.");
}
