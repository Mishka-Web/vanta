#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    row: usize,
    col: usize,
    preferred_col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            preferred_col: 0,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
        self.preferred_col = self.col;
    }

    pub fn move_right(&mut self, line_len: usize) {
        self.col = (self.col + 1).min(line_len.saturating_sub(1));
        self.preferred_col = self.col;
    }

    pub fn move_up(&mut self, target_line_len: usize) {
        if self.row > 0 {
            self.row -= 1;
        }
        self.col = self.preferred_col.min(target_line_len.saturating_sub(1));
    }

    pub fn move_down(&mut self, max_row: usize, target_line_len: usize) {
        self.row = (self.row + 1).min(max_row);
        self.col = self.preferred_col.min(target_line_len.saturating_sub(1));
    }

    pub fn set_position(&mut self, row: usize, col: usize, line_len: usize) {
        self.row = row;
        self.col = col.min(line_len.saturating_sub(1));
        self.preferred_col = self.col;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_starts_at_origin() {
        let cursor = Cursor::new();
        assert_eq!((cursor.row(), cursor.col()), (0, 0));
    }

    #[test]
    fn left_does_not_go_negative() {
        let mut cursor = Cursor::new();
        cursor.move_left();
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn right_is_clamped_to_line() {
        let mut cursor = Cursor::new();
        cursor.move_right(3);
        cursor.move_right(3);
        cursor.move_right(3);
        cursor.move_right(3);
        assert_eq!(cursor.col(), 2);
    }

    #[test]
    fn preferred_column_is_restored_after_short_line() {
        let mut cursor = Cursor::new();
        cursor.set_position(1, 5, 10);

        cursor.move_down(2, 3);
        assert_eq!(cursor.col(), 2);

        cursor.move_up(10);
        assert_eq!(cursor.col(), 5);
    }
}
