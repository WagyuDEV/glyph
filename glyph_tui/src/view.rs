use std::io::{stdout, Stdout};

use glyph_core::buffer::Buffer;
use glyph_core::config::{Config, LineNumbers};
use glyph_core::cursor::Cursor;
use glyph_core::editor::{Cell, Mode, Position, Rect};
use glyph_core::gutter::absolute_line_gutter::AbsoluteLineGutter;
use glyph_core::gutter::noop_line_gutter::NoopLineGutter;
use glyph_core::gutter::relative_line_gutter::RelativeLineGutter;
use glyph_core::gutter::Gutter;
use glyph_core::theme::Theme;
use glyph_core::ui::{Renderable, Scrollable};

use crossterm::{cursor, style, QueueableCommand};

use crate::diff::Viewport;

pub struct TuiView<'a> {
    stdout: Stdout,
    area: Rect,
    config: &'a Config,
    theme: &'a Theme,
    scroll: Position,
    diff: Viewport,
    gutter: Box<dyn Gutter>,
}

impl<'a> TuiView<'a> {
    pub fn new(area: Rect, config: &'a Config, theme: &'a Theme) -> Self {
        let gutter: Box<dyn Gutter> = match config.line_numbers {
            LineNumbers::Absolute => Box::new(AbsoluteLineGutter::new(
                config.empty_line_char,
                config.gutter_width,
            )),
            LineNumbers::Relative => Box::new(RelativeLineGutter::new(
                config.empty_line_char,
                config.gutter_width,
                config.line_numbers.clone(),
            )),
            LineNumbers::RelativeNumbered => Box::new(RelativeLineGutter::new(
                config.empty_line_char,
                config.gutter_width,
                config.line_numbers.clone(),
            )),
            LineNumbers::None => Box::<NoopLineGutter>::default(),
        };

        Self {
            stdout: stdout(),
            diff: Viewport::default(),
            area,
            config,
            scroll: Position::default(),
            gutter,
            theme,
        }
    }

    fn draw_sidebar(&mut self, buffer: &Buffer, cursor: &Cursor, diff: &mut Viewport) {
        let scroll = self.get_scroll().clone();
        let lines = self.gutter.get_lines(
            buffer.marker.len(),
            cursor.row,
            scroll.row,
            self.area.height,
        );
        for (row, line) in lines.iter().enumerate() {
            diff.set_text(0, row, line, &self.theme.gutter);
        }
    }

    fn draw_cursor(&mut self, mode: &Mode, buffer: &Buffer, cursor: &Cursor) -> anyhow::Result<()> {
        let offset = self.config.gutter_width;
        let scroll = &self.scroll;

        let col = {
            let mut col = 0;
            if let Some(mark) = buffer.marker.get_by_line(cursor.row + 1) {
                col = match mode {
                    Mode::Normal => cursor.col.min(mark.size.saturating_sub(2)),
                    _ => cursor.col.min(mark.size.saturating_sub(1)),
                };
            }
            col
        };

        self.stdout.queue(crossterm::cursor::MoveTo(
            col.saturating_sub(scroll.col) as u16 + offset as u16,
            cursor.row.saturating_sub(scroll.row) as u16,
        ))?;

        Ok(())
    }

    fn hide_cursor(&mut self) -> anyhow::Result<()> {
        self.stdout.queue(crossterm::cursor::Hide)?;
        Ok(())
    }

    fn show_cursor(&mut self) -> anyhow::Result<()> {
        self.stdout.queue(crossterm::cursor::Show)?;
        Ok(())
    }

    fn draw(&self, view: &mut Viewport, cells: &[Cell]) {
        let mut row = 0;
        let mut col = self.config.gutter_width;
        for cell in cells {
            if col >= self.scroll.col && col - self.scroll.col < self.area.width {
                // we print a space when the char is a newline so the background gets printed
                match cell.c {
                    '\n' => view.set_cell(col - self.scroll.col, row, ' ', &cell.style),
                    _ => view.set_cell(col - self.scroll.col, row, cell.c, &cell.style),
                };
                col += 1;
            }

            if cell.c == '\n' {
                row += 1;
                col = self.config.gutter_width;
            }
        }
    }
}

impl Scrollable for TuiView<'_> {}

impl Renderable for TuiView<'_> {
    fn render(
        &mut self,
        cells: &[Cell],
        buffer: &Buffer,
        cursor: &Cursor,
        mode: &Mode,
    ) -> anyhow::Result<()> {
        let default_style = &self.theme.style;
        let last_diff = self.diff.clone();
        let mut diff = Viewport::new(self.area.width, self.area.height);
        self.hide_cursor()?;
        self.draw(&mut diff, cells);
        self.draw_sidebar(buffer, cursor, &mut diff);
        let changes = diff.diff(&last_diff);

        for change in changes {
            let col = self.area.col + change.col;
            let row = self.area.row + change.row;

            self.stdout.queue(cursor::MoveTo(col as u16, row as u16))?;

            match change.cell.style.bg {
                Some(bg) => self.stdout.queue(style::SetBackgroundColor(bg))?,
                None => self
                    .stdout
                    .queue(style::SetBackgroundColor(default_style.bg.unwrap()))?,
            };

            match change.cell.style.fg {
                Some(fg) => self.stdout.queue(style::SetForegroundColor(fg))?,
                None => self
                    .stdout
                    .queue(style::SetForegroundColor(default_style.fg.unwrap()))?,
            };

            self.stdout.queue(style::Print(change.cell.c))?;
        }

        self.draw_cursor(mode, buffer, cursor)?;
        self.show_cursor()?;
        self.diff = diff;

        Ok(())
    }

    fn resize(&mut self, new_area: Rect) {
        self.area = new_area;
    }

    fn get_area(&self) -> &Rect {
        &self.area
    }

    fn get_scroll(&self) -> &Position {
        &self.scroll
    }

    fn set_scroll(&mut self, scroll: Position) {
        self.scroll = scroll;
    }
}
