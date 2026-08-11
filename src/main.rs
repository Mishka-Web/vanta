#![cfg_attr(not(windows), allow(dead_code))]

#[cfg(windows)]
mod cursor;
#[cfg(windows)]
mod input;
#[cfg(windows)]
mod screen;
#[cfg(windows)]
mod terminal;
#[cfg(windows)]
mod viewport;

#[cfg(windows)]
use cursor::Cursor;
#[cfg(windows)]
use input::{KeyCode, KeyEvent};
#[cfg(windows)]
use screen::Screen;
#[cfg(windows)]
use std::io;
#[cfg(windows)]
use viewport::Viewport;

#[cfg(windows)]
const DEMO_LINES: &[&str] = &[
    "fn main() {",
    "    println!(\"VANTA\");",
    "}",
    "",
    "// Keyboard-first editor",
    "// v0.0.4 Editor Cursor",
    "",
    "const NAME: &str = \"VANTA\";",
    "",
    "struct Editor {",
    "    cursor: Cursor,",
    "    viewport: Viewport,",
    "}",
    "",
    "impl Editor {",
    "    fn run(&mut self) {",
    "        // coming soon",
    "    }",
    "}",
    "",
    "// h j k l",
    "// arrows",
    "// scrolling",
    "",
    "EOF",
];

#[cfg(windows)]
fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("vanta {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("VANTA {}", env!("CARGO_PKG_VERSION"));
        println!("Keyboard-first modal editor");
        println!();
        println!("USAGE:");
        println!("    vanta [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -h, --help       Print help");
        println!("    -V, --version    Print version");
        return Ok(());
    }

    let _terminal = terminal::Terminal::enter()?;
    let (width, height) = terminal::terminal_size()?;
    let mut screen = Screen::new(width, height);

    let editor_height = height.saturating_sub(2).max(1) as usize;
    let mut cursor = Cursor::new();
    let mut viewport = Viewport::new(editor_height);

    draw_ui(&mut screen, &cursor, &viewport, None);
    terminal::render(&screen)?;

    loop {
        let key = input::read_key()?;

        if matches!(key.code, KeyCode::Char('q')) && key.modifiers.is_empty() {
            break;
        }

        let current_row = cursor.row();
        let current_len = line_len(current_row);

        match key.code {
            KeyCode::Left | KeyCode::Char('h') if key.modifiers.is_empty() => {
                cursor.move_left();
            }
            KeyCode::Right | KeyCode::Char('l') if key.modifiers.is_empty() => {
                cursor.move_right(current_len);
            }
            KeyCode::Up | KeyCode::Char('k') if key.modifiers.is_empty() => {
                let target_row = current_row.saturating_sub(1);
                cursor.move_up(line_len(target_row));
            }
            KeyCode::Down | KeyCode::Char('j') if key.modifiers.is_empty() => {
                let max_row = DEMO_LINES.len().saturating_sub(1);
                let target_row = (current_row + 1).min(max_row);
                cursor.move_down(max_row, line_len(target_row));
            }
            KeyCode::Home => {
                cursor.set_position(current_row, 0, current_len);
            }
            KeyCode::End => {
                cursor.set_position(current_row, current_len.saturating_sub(1), current_len);
            }
            _ => {}
        }

        let (new_width, new_height) = terminal::terminal_size()?;
        if new_width != screen.width() || new_height != screen.height() {
            screen.resize(new_width, new_height);
            viewport.resize(new_height.saturating_sub(2).max(1) as usize);
        }

        viewport.ensure_visible(cursor.row());
        draw_ui(&mut screen, &cursor, &viewport, Some(key));
        terminal::render(&screen)?;
    }

    Ok(())
}

#[cfg(windows)]
fn line_len(row: usize) -> usize {
    DEMO_LINES
        .get(row)
        .map(|line| line.chars().count().max(1))
        .unwrap_or(1)
}

#[cfg(windows)]
fn draw_ui(
    screen: &mut Screen,
    cursor: &Cursor,
    viewport: &Viewport,
    last_key: Option<KeyEvent>,
) {
    screen.clear();

    let editor_height = screen.height().saturating_sub(2) as usize;

    for screen_row in 0..editor_height {
        let doc_row = viewport.top() + screen_row;
        if let Some(line) = DEMO_LINES.get(doc_row) {
            let number = format!("{:>4} ", doc_row + 1);
            screen.write_str(screen_row as u16, 0, &number);
            screen.write_str(screen_row as u16, 5, line);
        } else {
            screen.write_str(screen_row as u16, 0, "   ~ ");
        }
    }

    let status_row = screen.height().saturating_sub(2);
    let status = format!(
        " VANTA v0.0.5  NORMAL  Ln {}, Col {}  top={} ",
        cursor.row() + 1,
        cursor.col() + 1,
        viewport.top() + 1
    );
    screen.write_str(status_row, 0, &status);

    let message_row = screen.height().saturating_sub(1);
    let message = match last_key {
        Some(key) => format!(
            " {:?} [S:{} C:{} A:{}] | h/j/k/l or arrows | q quit",
            key.code,
            key.modifiers.shift(),
            key.modifiers.ctrl(),
            key.modifiers.alt()
        ),
        None => " h/j/k/l or arrows | Home/End | q quit".to_string(),
    };
    screen.write_str(message_row, 0, &message);

    let visible_row = cursor.row().saturating_sub(viewport.top());
    let screen_cursor_row = visible_row.min(editor_height.saturating_sub(1)) as u16;
    let screen_cursor_col = (5 + cursor.col()).min(screen.width().saturating_sub(1) as usize) as u16;

    screen.set_cursor(screen_cursor_row, screen_cursor_col);
}

#[cfg(not(windows))]
fn main() {
    eprintln!("VANTA v0.0.5 currently targets Windows only.");
}
