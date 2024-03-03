use crate::gutter::Gutter;

#[derive(Debug, Default)]
pub struct NoopLineGutter {}

impl Gutter for NoopLineGutter {
    fn get_lines(
        &self,
        _total_lines: usize,
        _line: usize,
        _scroll: usize,
        _height: usize,
    ) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_does_nothing() {
        let noop_gutter = NoopLineGutter::default();
        noop_gutter.get_lines(2, 1, 0, 0);
    }
}
