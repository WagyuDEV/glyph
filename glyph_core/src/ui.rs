use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::editor::{Cell, Mode, Position, Rect};

pub trait Scrollable: Renderable {
    fn maybe_scroll(&mut self, cursor: &Cursor) {
        let Rect { width, height, .. } = self.get_area();
        let mut scroll = self.get_scroll().clone();
        // all the instances of `y + 1` or `x + 1` are just normalizing the row/col to be 1 indexed
        match (cursor.col, cursor.row) {
            // should scroll down
            (_, y) if (y + 1).saturating_sub(scroll.row) >= *height => {
                scroll.row = y + 1 - height;
            }
            // Should scroll up
            (_, y) if (y + 1).saturating_sub(scroll.row) == 0 => {
                scroll.row = scroll.row - (scroll.row - y);
            }
            // Should scroll right
            (x, _) if x.saturating_sub(scroll.col) >= *width => {
                scroll.col = x + 1 - width;
            }
            // Should scroll left
            (x, _) if (x + 1).saturating_sub(scroll.col) == 0 => {
                scroll.col = scroll.col - (scroll.col - x);
            }
            _ => (),
        }
        self.set_scroll(scroll.clone());
    }
}

pub trait Renderable {
    fn render(
        &mut self,
        cells: &[Cell],
        buffer: &Buffer,
        cursor: &Cursor,
        mode: &Mode,
    ) -> anyhow::Result<()>;
    fn resize(&mut self, new_area: Rect);
    fn get_area(&self) -> &Rect;
    fn get_scroll(&self) -> &Position;
    fn set_scroll(&mut self, scroll: Position);
}
