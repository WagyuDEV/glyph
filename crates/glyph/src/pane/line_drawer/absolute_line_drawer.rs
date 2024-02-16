use crate::config::Config;
use crate::pane::{LineDrawer, PaneDimensions};
use crossterm::style::{Color, Print, Stylize};
use crossterm::{cursor, QueueableCommand};
use std::io;

#[derive(Debug)]
pub struct AbsoluteLineDrawer {
    stdout: io::Stdout,
    config: &'static Config,
}

impl AbsoluteLineDrawer {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            config: Config::get(),
        }
    }
}

impl LineDrawer for AbsoluteLineDrawer {
    fn draw_lines(
        &mut self,
        dimensions: &PaneDimensions,
        total_lines: u16,
        _: u16,
        scroll_row: u16,
    ) -> io::Result<()> {
        let total_lines = u16::min(dimensions.height, total_lines);
        let mut scroll_row = scroll_row;

        for i in 0..total_lines {
            scroll_row += 1;
            let line = scroll_row.to_string();
            let offset = dimensions.col + self.config.sidebar_width - line.len() as u16;

            self.stdout
                .queue(cursor::MoveTo(offset, i))?
                .queue(Print(line.with(Color::DarkGrey)))?;
        }

        if total_lines < dimensions.height {
            for i in total_lines..dimensions.height {
                let offset = dimensions.col + self.config.sidebar_width - 1;
                self.stdout
                    .queue(cursor::MoveTo(offset, i))?
                    .queue(Print(self.config.empty_line_char))?;
            }
        }

        Ok(())
    }
}
