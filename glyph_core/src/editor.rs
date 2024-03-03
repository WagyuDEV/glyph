use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::buffer::Buffer;
use crate::theme::Theme;
use crate::window::{Position, Rect};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Search,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Search => f.write_str("SEARCH"),
            Self::Insert => f.write_str("INSERT"),
            Self::Normal => f.write_str("NORMAL"),
            Self::Command => f.write_str("COMMAND"),
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Self {
            width: width as usize,
            height: height as usize,
        }
    }
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

pub trait Statusline<'a> {
    fn new(area: Rect, theme: &'a Theme) -> Self;
    fn resize(area: Rect);
    fn render(&mut self, update: StatuslineUpdate) -> anyhow::Result<()>;
}

pub trait Commandline {
    fn new(area: Rect) -> Self;
    fn render(&mut self) -> anyhow::Result<()>;
}
