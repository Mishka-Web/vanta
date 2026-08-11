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
    pub fn set_cursor(&mut self, row: u16, col: u16) {
        self.cursor_row = row.min(self.height.saturating_sub(1));
        self.cursor_col = col.min(self.width.saturating_sub(1));
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
        self.cells[row as usize * self.width as usize + col as usize] = Cell::new(ch);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn screen_starts_with_requested_size() {
        let s = Screen::new(80, 24);
        assert_eq!(s.cells().len(), 80 * 24);
    }
    #[test]
    fn write_str_writes_cells() {
        let mut s = Screen::new(10, 2);
        s.write_str(0, 0, "VANTA");
        let t: String = s.cells()[0..5].iter().map(|c| c.ch()).collect();
        assert_eq!(t, "VANTA");
    }
    #[test]
    fn set_cursor_is_clamped_to_screen() {
        let mut s = Screen::new(5, 3);
        s.set_cursor(99, 99);
        assert_eq!((s.cursor_row(), s.cursor_col()), (2, 4));
    }
}
