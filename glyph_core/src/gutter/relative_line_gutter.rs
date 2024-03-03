use crate::config::LineNumbers;
use crate::gutter::Gutter;

#[derive(Debug)]
pub struct RelativeLineGutter {
    empty_line_char: char,
    offset: usize,
    line_numbers: LineNumbers,
}

impl RelativeLineGutter {
    pub fn new(empty_line_char: char, offset: usize, line_numbers: LineNumbers) -> Self {
        Self {
            empty_line_char,
            offset,
            line_numbers,
        }
    }
}

impl Gutter for RelativeLineGutter {
    fn get_lines(
        &self,
        total_lines: usize,
        line: usize,
        scroll: usize,
        height: usize,
    ) -> Vec<String> {
        let total_lines = usize::min(height, total_lines);
        let normalized_line = line + 1;
        let mut scroll_row = scroll;
        let mut lines = vec![];

        for _ in 0..total_lines {
            scroll_row += 1;
            let mut line = usize::abs_diff(scroll_row, normalized_line).to_string();

            if let LineNumbers::RelativeNumbered = self.line_numbers {
                match normalized_line {
                    l if l == scroll_row => line = scroll_row.to_string(),
                    _ => (),
                }
            }

            line = " ".repeat(self.offset - 1 - line.len()) + &line;
            line.push(' ');
            lines.push(line);
        }

        if total_lines < height {
            let mut line = " ".repeat(self.offset - 2);
            line.push(self.empty_line_char);
            line.push(' ');
            lines.push(line);
        }

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_gutter() {
        let relative_gutter = RelativeLineGutter::new('~', 6, LineNumbers::Relative);

        relative_gutter.get_lines(3, 2, 0, 0);
    }

    #[test]
    fn test_draw_with_scroll() {
        let relative_gutter = RelativeLineGutter::new('~', 6, LineNumbers::Relative);

        relative_gutter.get_lines(400, 103, 103, 0);
    }

    #[test]
    fn test_draw_with_scroll_numbered() {
        let relative_gutter = RelativeLineGutter::new('~', 6, LineNumbers::RelativeNumbered);

        relative_gutter.get_lines(400, 103, 103, 100);
    }
}
