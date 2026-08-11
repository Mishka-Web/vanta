#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    ch: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ' }
    }
}

impl Cell {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }

    pub fn ch(self) -> char {
        self.ch
    }
}

pub struct Screen {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
    cursor_row: u16,
    cursor_col: u16,
}

impl Screen {
    pub fn new(width: u16, height: u16) -> Self {
        let width = width.max(1);
        let height = height.max(1);

        Self {
            width,
            height,
            cells: vec![Cell::default(); width as usize * height as usize],
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn cursor_row(&self) -> u16 {
        self.cursor_row
    }

    pub fn cursor_col(&self) -> u16 {
        self.cursor_col
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let width = width.max(1);
        let height = height.max(1);

        self.width = width;
        self.height = height;
        self.cells = vec![Cell::default(); width as usize * height as usize];

        self.cursor_row = self.cursor_row.min(height.saturating_sub(1));
        self.cursor_col = self.cursor_col.min(width.saturating_sub(1));
    }

    pub fn set(&mut self, row: u16, col: u16, ch: char) {
        if row >= self.height || col >= self.width {
            return;
        }

        let index = row as usize * self.width as usize + col as usize;
        self.cells[index] = Cell::new(ch);
    }

    pub fn write_str(&mut self, row: u16, col: u16, text: &str) {
        if row >= self.height || col >= self.width {
            return;
        }

        for (current_col, ch) in (col..).zip(text.chars()) {
            if current_col >= self.width {
                break;
            }

            self.set(row, current_col, ch);
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_col = self.cursor_col.saturating_sub(1);
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_col = (self.cursor_col + 1).min(self.width.saturating_sub(1));
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_row = self.cursor_row.saturating_sub(1);
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_row = (self.cursor_row + 1).min(self.height.saturating_sub(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_starts_with_requested_size() {
        let screen = Screen::new(80, 24);
        assert_eq!(screen.width(), 80);
        assert_eq!(screen.height(), 24);
        assert_eq!(screen.cells().len(), 80 * 24);
    }

    #[test]
    fn write_str_writes_cells() {
        let mut screen = Screen::new(10, 2);
        screen.write_str(0, 0, "VANTA");

        let text: String = screen.cells()[0..5].iter().map(|cell| cell.ch()).collect();

        assert_eq!(text, "VANTA");
    }

    #[test]
    fn cursor_does_not_leave_screen() {
        let mut screen = Screen::new(2, 2);

        screen.move_cursor_left();
        screen.move_cursor_up();
        assert_eq!((screen.cursor_row(), screen.cursor_col()), (0, 0));

        screen.move_cursor_right();
        screen.move_cursor_right();
        screen.move_cursor_down();
        screen.move_cursor_down();

        assert_eq!((screen.cursor_row(), screen.cursor_col()), (1, 1));
    }
}
