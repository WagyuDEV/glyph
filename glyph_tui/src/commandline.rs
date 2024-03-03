use glyph_core::commandline::Commandline;
use glyph_core::editor::Rect;

#[derive(Debug, Default)]
pub struct TuiCommandline {}

impl Commandline for TuiCommandline {
    fn new(area: Rect) -> Self {
        Self {}
    }
    fn render(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
