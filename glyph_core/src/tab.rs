use crate::config::KeyAction;
use crate::editor::Mode;
use crate::lsp::IncomingMessage;
use crate::window::Rect;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::*;

    use crate::config::{Config, EditorBackground, Keys, LineNumbers};
    use crate::theme::Theme;

    fn get_config() -> Config {
        Config {
            gutter_width: 6,
            theme: "".into(),
            keys: Keys::default(),
            log_file: None,
            background: EditorBackground::Dark,
            line_numbers: LineNumbers::Absolute,
            empty_line_char: '~',
            show_diagnostics: true,
            mouse_scroll_lines: None,
        }
    }

    #[test]
    fn test_resizing() {
        // let buffer = Buffer::new(1, None).unwrap();
        // let theme = Theme::default();
        // let config = get_config();
        // let pane = Pane::new(1, Arc::new(Mutex::new(buffer)), &theme, &config);
        // let mut window = Window::new(1);

        // assert_eq!(
        //     window.get_active_pane().size,
        //     Rect {
        //         row: 0,
        //         col: 0,
        //         height: 1,
        //         width: 1
        //     }
        // );

        // window.resize((0, 0).into(), &Mode::Normal).unwrap();

        // assert_eq!(
        //     window.get_active_pane().size,
        //     Rect {
        //         row: 0,
        //         col: 0,
        //         height: 0,
        //         width: 0
        //     }
        // );
    }
}
