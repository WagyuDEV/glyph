use std::{cell::RefCell, rc::Rc};

use crate::buffer::Buffer;
use crate::editor::{Mode, Position, Rect};
use crate::theme::Theme;

pub trait Statusline<'a> {
    fn new(area: Rect, theme: &'a Theme) -> Self;
    fn resize(area: Rect);
    fn render(&mut self, update: StatuslineUpdate) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct StatuslineUpdate {
    pub mode: Mode,
    pub cursor_pos: Position,
    pub buffer: Rc<RefCell<Buffer>>,
}

impl StatuslineUpdate {
    pub fn new(mode: Mode, cursor_pos: Position, buffer: Rc<RefCell<Buffer>>) -> Self {
        Self {
            mode,
            cursor_pos,
            buffer,
        }
    }
}
