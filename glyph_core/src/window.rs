use std::cell::RefCell;
use std::rc::Rc;

use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::editor::{Action, Cell, Mode, Position, Rect};
use crate::highlight::Highlight;
use crate::theme::Theme;
use crate::ui::Scrollable;

pub struct Window<'a> {
    pub id: usize,
    pub cursor: Cursor,
    highlight: Highlight<'a>,
    view: Box<dyn Scrollable + 'a>,
    pub buffer: Option<Rc<RefCell<Buffer>>>,
    // Currently, `layers[0]` is the buffer layer and `layers[1]` is the popups layer
    pub size: Rect,
    theme: &'a Theme,
}

impl<'a> Window<'a> {
    pub fn new(
        id: usize,
        buffer: Option<Rc<RefCell<Buffer>>>,
        theme: &'a Theme,
        size: Rect,
        view: Box<dyn Scrollable + 'a>,
    ) -> Self {
        Self {
            id,
            buffer,
            highlight: Highlight::new(theme),
            cursor: Cursor::default(),
            view,
            size,
            theme,
        }
    }

    pub fn initialize(&mut self, mode: &Mode) -> anyhow::Result<()> {
        self.render(mode)?;
        Ok(())
    }

    pub fn resize(&mut self, new_size: Rect, mode: &Mode) -> anyhow::Result<()> {
        self.size = new_size.clone();
        self.view.resize(new_size);
        self.render(mode)?;
        Ok(())
    }

    pub fn handle_action(&mut self, action: &Action, mode: &Mode) -> anyhow::Result<()> {
        let col = self.cursor.col;
        let row = self.cursor.row;
        let mark = {
            let buffer = self.buffer.as_mut().unwrap().borrow_mut();
            let mark = buffer.marker.get_by_cursor(self.cursor.absolute_position);
            mark.unwrap()
        };

        {
            let mut buffer = self.buffer.as_mut().unwrap().borrow_mut();
            buffer.handle_action(action, self.cursor.absolute_position)?;
            self.cursor.handle_action(action, &mut buffer, mode);
        }

        if let Action::DeletePreviousChar = action {
            if let (0, 1..) = (col, row) {
                self.cursor.col = mark.size.saturating_sub(1);
                self.cursor.absolute_position = mark.start + mark.size.saturating_sub(1);
            }
        };

        self.render(mode)?;
        Ok(())
    }

    fn render(&mut self, mode: &Mode) -> anyhow::Result<()> {
        self.view.maybe_scroll(&self.cursor);
        let cells = self.get_highlight();
        self.view.render(
            &cells,
            &self.buffer.as_ref().unwrap().borrow(),
            &self.cursor,
            mode,
        )?;
        Ok(())
    }

    fn get_highlight(&mut self) -> Vec<Cell> {
        let mut result: Vec<Cell> = Vec::new();
        let mut current_byte_index = 0;
        let scroll = self.view.get_scroll();
        let buffer = self
            .buffer
            .as_ref()
            .unwrap()
            .borrow()
            .content_from(scroll.row, self.size.height);
        let colors = self.highlight.colors(&buffer);
        let style = self.theme.style;

        for c in buffer.chars() {
            let cell = match colors
                .iter()
                .find(|token| current_byte_index >= token.start && current_byte_index < token.end)
            {
                Some(token) => Cell {
                    c,
                    style: *token.style,
                },
                None => Cell { c, style },
            };
            result.push(cell);
            current_byte_index += c.len_utf8();
        }

        result
    }

    pub fn get_cursor_readable_position(&self) -> Position {
        self.cursor.get_readable_position()
    }

    pub fn get_buffer(&self) -> Rc<RefCell<Buffer>> {
        self.buffer.as_ref().unwrap().clone()
    }
}
