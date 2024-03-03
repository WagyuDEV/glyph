use std::io::{stdout, Stdout};

use crossterm::{
    cursor,
    style::{self, Print},
    QueueableCommand,
};
use glyph_core::{
    editor::{Position, Rect},
    statusline::{Statusline, StatuslineUpdate},
    theme::Theme,
};

use crate::diff::Viewport;

#[derive(Debug)]
pub struct TuiStatusline<'a> {
    area: Rect,
    view: Viewport,
    theme: &'a Theme,
    stdout: Stdout,
}

impl<'a> Statusline<'a> for TuiStatusline<'a> {
    fn new(area: Rect, theme: &'a Theme) -> Self {
        Self {
            view: Viewport::new(area.width, area.height),
            area,
            theme,
            stdout: stdout(),
        }
    }

    fn resize(area: Rect) {}

    fn render(&mut self, update: StatuslineUpdate) -> anyhow::Result<()> {
        self.draw(&update);
        for (x, cell) in self.view.cells.iter().enumerate() {
            self.stdout
                .queue(cursor::MoveTo(x as u16, self.area.row as u16))?;

            if let Some(bg) = cell.style.bg {
                self.stdout.queue(style::SetBackgroundColor(bg))?;
            } else {
                self.stdout
                    .queue(style::SetBackgroundColor(self.theme.style.bg.unwrap()))?;
            }
            if let Some(fg) = cell.style.fg {
                self.stdout.queue(style::SetForegroundColor(fg))?;
            } else {
                self.stdout
                    .queue(style::SetForegroundColor(self.theme.style.fg.unwrap()))?;
            }

            self.stdout.queue(Print(cell.c))?;
        }
        Ok(())
    }
}

impl TuiStatusline<'_> {
    fn draw(&mut self, update: &StatuslineUpdate) {
        let buffer = update.buffer.borrow();
        let lines = buffer.marker.len();
        let mode = &update.mode;
        let Position { col, row } = update.cursor_pos;
        let file_name = buffer.file_name.clone();

        let cursor = format!("{}:{} ", row, col);
        let percentage = match row {
            1 => "TOP ".into(),
            _ if row == lines => "BOT ".into(),
            _ => format!("{}% ", (row as f64 / lines as f64 * 100.0) as usize),
        };

        let file_name = file_name.split('/').rev().nth(0).unwrap();
        let file_name = format!(" {}", file_name);

        let mode = format!(" {}", mode);

        let padding = " ".repeat(
            self.area.width - mode.len() - file_name.len() - cursor.len() - percentage.len(),
        );
        self.view
            .set_text(0, 0, &mode, &self.theme.statusline.inner);
        self.view
            .set_text(mode.len(), 0, &file_name, &self.theme.statusline.inner);
        self.view.set_text(
            mode.len() + file_name.len(),
            0,
            &padding,
            &self.theme.statusline.inner,
        );

        self.view.set_text(
            self.area.width - 1 - cursor.len(),
            0,
            &cursor,
            &self.theme.statusline.inner,
        );

        self.view.set_text(
            self.area.width - cursor.len(),
            0,
            &cursor,
            &self.theme.statusline.inner,
        );

        self.view.set_text(
            self.area.width - cursor.len() - percentage.len(),
            0,
            &percentage,
            &self.theme.statusline.inner,
        );
    }
}
