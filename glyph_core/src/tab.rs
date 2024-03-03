use crate::editor::KeyAction;
use crate::editor::Mode;
use crate::editor::Rect;
use crate::lsp::IncomingMessage;

pub struct Tab {
    pub id: usize,
}

impl Tab {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn resize(&mut self, new_size: Rect, mode: &Mode) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn handle_action(&mut self, action: &KeyAction, mode: &Mode) -> anyhow::Result<()> {
        if let KeyAction::Simple(_) = action {}
        Ok(())
    }

    pub fn handle_lsp_message(
        &mut self,
        message: (IncomingMessage, Option<String>),
        mode: &Mode,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn initialize(&mut self, mode: &Mode) -> anyhow::Result<()> {
        Ok(())
    }
}
