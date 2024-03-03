use crate::gutter::Gutter;

#[derive(Debug, Clone)]
pub struct AbsoluteLineGutter {
    empty_line_char: char,
    offset: usize,
}

impl AbsoluteLineGutter {
    pub fn new(empty_line_char: char, offset: usize) -> Self {
        Self {
            empty_line_char,
            offset,
        }
    }
}

impl Gutter for AbsoluteLineGutter {
    fn get_lines(
        &self,
        total_lines: usize,
        _current_line: usize,
        scroll: usize,
        height: usize,
    ) -> Vec<String> {
        let total_lines = usize::min(height, total_lines);
        let mut scroll = scroll;
        let mut lines = vec![];

        for _ in 0..total_lines {
            scroll += 1;
            let mut line = scroll.to_string();
            line = " ".repeat(self.offset - 1 - line.len()) + &line;
            line.push(' ');
            lines.push(line);
        }

        if total_lines < height {
            for _ in total_lines..height {
                let mut line = " ".repeat(self.offset - 2);
                line.push(self.empty_line_char);
                line.push(' ');
                lines.push(line);
            }
        }

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_gutter() {
        let absolute_gutter = AbsoluteLineGutter::new('~', 6);

        absolute_gutter.get_lines(3, 2, 0, 10);
    }

    #[test]
    fn test_draw_with_scroll() {
        let absolute_gutter = AbsoluteLineGutter::new('~', 6);
        absolute_gutter.get_lines(400, 0, 103, 10);
    }
}
