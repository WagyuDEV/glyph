use crate::editor::KeyAction;
use crate::editor::Mode;

use crossterm::event::Event;

pub trait EventHandler {
    // TODO: once we start looking into GUI, this would have to be our own event
    fn poll(&mut self, event: &Event, mode: &Mode) -> Option<KeyAction>;
}
