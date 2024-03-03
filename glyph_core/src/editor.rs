use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::theme::Style;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum KeyAction {
    Simple(Action),
    Multiple(Vec<Action>),
    Complex(HashMap<String, KeyAction>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    EnterMode(Mode),
    Quit,
    Undo,
    InsertLine,
    InsertLineBelow,
    InsertLineAbove,
    PasteBelow,
    FindNext,
    FindPrevious,
    CenterLine,
    InsertTab,
    InsertChar(char),
    InsertCommand(char),
    ExecuteCommand,
    SaveBuffer,
    DeleteUntilEOL,
    Resize(u16, u16),

    NextWord,
    PreviousWord,
    MoveLeft,
    MoveDown,
    MoveUp,
    MoveRight,
    MoveToBottom,
    MoveToTop,
    MoveToLineEnd,
    MoveToLineStart,
    PageDown,
    PageUp,

    DeleteCurrentChar,
    DeleteBack,
    DeleteWord,
    DeleteLine,
    DeletePreviousChar,

    GoToDefinition,
    Hover,
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

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rect {
    pub row: usize,
    pub col: usize,
    pub height: usize,
    pub width: usize,
}

impl Rect {
    pub fn new(col: usize, row: usize, width: usize, height: usize) -> Self {
        Self {
            col,
            row,
            width,
            height,
        }
    }
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Self {
            col: 0,
            row: 0,
            width: size.width,
            height: size.height,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub c: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            style: Default::default(),
        }
    }
}
