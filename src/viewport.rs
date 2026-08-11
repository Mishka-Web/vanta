#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    top: usize,
    height: usize,
}

impl Viewport {
    pub fn new(height: usize) -> Self {
        Self {
            top: 0,
            height: height.max(1),
        }
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn resize(&mut self, height: usize) {
        self.height = height.max(1);
    }

    pub fn ensure_visible(&mut self, row: usize) {
        if row < self.top {
            self.top = row;
            return;
        }

        let bottom_exclusive = self.top + self.height;
        if row >= bottom_exclusive {
            self.top = row + 1 - self.height;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_scrolls_down_to_cursor() {
        let mut viewport = Viewport::new(5);
        viewport.ensure_visible(7);
        assert_eq!(viewport.top(), 3);
    }

    #[test]
    fn viewport_scrolls_up_to_cursor() {
        let mut viewport = Viewport::new(5);
        viewport.ensure_visible(10);
        viewport.ensure_visible(2);
        assert_eq!(viewport.top(), 2);
    }
}
