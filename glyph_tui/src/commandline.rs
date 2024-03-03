use glyph_core::editor::Commandline;

#[derive(Debug, Default)]
pub struct TuiCommandline {}

impl Commandline for TuiCommandline {
    fn new(area: glyph_core::window::Rect) -> Self {
        Self {}
    }
    fn render(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
