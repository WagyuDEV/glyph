use crate::editor::Rect;

pub trait Commandline {
    fn new(area: Rect) -> Self;
    fn render(&mut self) -> anyhow::Result<()>;
}
