use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};
use std::rc::Rc;
use std::time::Duration;

use crossterm::event::EventStream;
use crossterm::{cursor, terminal, QueueableCommand};
use futures::{future::FutureExt, StreamExt};
use serde::{Deserialize, Serialize};

use crate::buffer::Buffer;
use crate::config::{Action, Config, KeyAction};
use crate::events::EventHandler;
use crate::lsp::{IncomingMessage, LspClient};
use crate::tab::Tab;
use crate::theme::Theme;
use crate::window::{Rect, Window};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Search,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Search => f.write_str("SEARCH"),
            Self::Insert => f.write_str("INSERT"),
            Self::Normal => f.write_str("NORMAL"),
            Self::Command => f.write_str("COMMAND"),
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Self {
            width: width as usize,
            height: height as usize,
        }
    }
}

pub trait Statusline {}
pub trait Commandline {}

pub struct Editor<'a /*S, C*/, E>
where
    // S: Statusline,
    // C: Commandline,
    E: EventHandler,
{
    event_handler: E,
    lsp: LspClient,
    stdout: Stdout,
    size: Size,
    //    statusline: S,
    //    commandline: C,
    mode: Mode,
    tabs: HashMap<usize, Tab>,
    windows: HashMap<usize, Window<'a>>,
    buffers: HashMap<usize, Rc<RefCell<Buffer>>>,
    active_tab: usize,
    active_window: usize,
    active_buffer: usize,
}

impl<'a /*S, C*/, E> Editor<'a /*S, C*/, E>
where
    // S: Statusline,
    // C: Commandline,
    E: EventHandler,
{
    pub async fn new(
        config: &'a Config,
        theme: &'a Theme,
        lsp: LspClient,
        file_name: Option<String>,
        // statusline: S,
        // commandline: C,
        event_handler: E,
    ) -> anyhow::Result<Self> {
        let mut editor = Self {
            event_handler,
            lsp,
            mode: Mode::Normal,
            stdout: stdout(),
            size: terminal::size()?.into(),
            // statusline,
            // commandline,
            tabs: HashMap::new(),
            windows: HashMap::new(),
            buffers: HashMap::new(),
            active_tab: 1,
            active_window: 1,
            active_buffer: 1,
        };

        let buffer_id = 1;
        let buffer = Rc::new(RefCell::new(Buffer::new(buffer_id, file_name)?));
        let mut window_size: Rect = editor.size.into();
        window_size.height -= 2;
        let window = Window::new(1, Some(buffer.clone()), theme, config, window_size);
        let tab = Tab::new(1);
        editor.tabs.insert(tab.id, tab);
        editor.windows.insert(window.id, window);
        editor.buffers.insert(buffer_id, buffer.clone());

        editor.start().await?;

        Ok(editor)
    }

    fn initialize(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout.queue(terminal::EnterAlternateScreen)?;
        self.tabs
            .get_mut(&self.active_tab)
            .unwrap()
            .initialize(&self.mode)?;
        self.windows
            .get_mut(&self.active_window)
            .unwrap()
            .initialize(&self.mode)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.initialize()?;
        self.lsp.initialize().await?;

        let mut stream = EventStream::new();
        loop {
            let delay = futures_timer::Delay::new(Duration::from_millis(30)).fuse();
            let event = stream.next().fuse();

            tokio::select! {
                _ = delay => {
                    if let Some(message) = self.lsp.try_read_message().await? {
                        self.handle_lsp_message(message)?;
                    }
                }
                maybe_event = event => {
                    if let Some(Ok(event)) = maybe_event {
                        if let Some(action) = self.event_handler.poll(&event, &self.mode) {
                            match action {
                                KeyAction::Simple(Action::Quit) => {
                                    break;
                                }
                                _ => self.handle_action(action).await?,
                            }
                        }
                    };
                }
            }
        }

        Ok(())
    }

    async fn handle_action(&mut self, action: KeyAction) -> anyhow::Result<()> {
        let mut actions = Vec::new();
        flatten_actions(&mut actions, action);

        let window = self.windows.get_mut(&self.active_window).unwrap();
        for action in actions {
            match action {
                Action::MoveToLineStart => window.handle_action(&action, &self.mode)?,
                Action::MoveToLineEnd => window.handle_action(&action, &self.mode)?,
                Action::DeletePreviousChar => window.handle_action(&action, &self.mode)?,
                Action::DeleteCurrentChar => window.handle_action(&action, &self.mode)?,
                Action::NextWord => window.handle_action(&action, &self.mode)?,
                Action::MoveLeft => window.handle_action(&action, &self.mode)?,
                Action::MoveDown => window.handle_action(&action, &self.mode)?,
                Action::MoveUp => window.handle_action(&action, &self.mode)?,
                Action::MoveRight => window.handle_action(&action, &self.mode)?,
                Action::MoveToTop => window.handle_action(&action, &self.mode)?,
                Action::SaveBuffer => window.handle_action(&action, &self.mode)?,
                Action::MoveToBottom => window.handle_action(&action, &self.mode)?,
                Action::InsertLine => window.handle_action(&action, &self.mode)?,
                Action::InsertLineBelow => window.handle_action(&action, &self.mode)?,
                Action::InsertLineAbove => window.handle_action(&action, &self.mode)?,
                Action::InsertChar(_) => window.handle_action(&action, &self.mode)?,
                Action::EnterMode(Mode::Insert) => {
                    self.mode = Mode::Insert;
                    self.stdout.queue(cursor::SetCursorStyle::SteadyBar)?;
                }
                Action::EnterMode(Mode::Normal) => {
                    self.mode = Mode::Normal;
                    // self.maybe_leave_command_mode()?;
                    self.stdout.queue(cursor::SetCursorStyle::SteadyBlock)?;
                }
                Action::EnterMode(Mode::Command) => {
                    self.mode = Mode::Command;
                    // self.enter_command_mode()?;
                    self.stdout.queue(cursor::SetCursorStyle::SteadyBar)?;
                }
                Action::Hover => {
                    let buffer = self.buffers.get(&self.active_buffer).unwrap();
                    let file_name = buffer.borrow().file_name.clone();
                    let row = window.cursor.row;
                    let col = window.cursor.col;
                    self.lsp.request_hover(&file_name, row, col).await?;
                }
                Action::Resize(cols, rows) => {
                    self.size = (cols, rows).into();
                    window.resize(
                        Rect {
                            row: 0,
                            col: 0,
                            height: self.size.height - 2,
                            width: self.size.width,
                        },
                        &self.mode,
                    )?;
                }
                _ => (),
            };
        }
        self.stdout.flush()?;
        Ok(())
    }

    fn handle_lsp_message(
        &mut self,
        _message: (IncomingMessage, Option<String>),
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

fn flatten_actions(actions: &mut Vec<Action>, action: KeyAction) {
    match action {
        KeyAction::Multiple(a) => actions.extend(a),
        KeyAction::Simple(a) => actions.push(a),
        KeyAction::Complex(map) => {
            map.values()
                .for_each(|a| flatten_actions(actions, a.clone()));
        }
    };
}
