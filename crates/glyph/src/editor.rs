use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossterm::event::EventStream;
use futures::{future::FutureExt, StreamExt};
use serde::{Deserialize, Serialize};

use crate::buffer::Buffer;
use crate::config::{Action, Config, KeyAction};
use crate::events::Events;
use crate::lsp::LspClient;
use crate::pane::Pane;
use crate::theme::Theme;
use crate::view::View;
use crate::window::Window;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Search,
}

pub struct Editor<'a> {
    events: Events<'a>,
    view: View<'a>,
    lsp: &'a LspClient,
    config: &'a Config,
    theme: &'a Theme,
    mode: Mode,
}

impl<'a> Editor<'a> {
    pub fn new(
        config: &'a Config,
        theme: &'a Theme,
        lsp: &'a LspClient,
        file_name: Option<String>,
    ) -> anyhow::Result<Self> {
        let buffer = Rc::new(RefCell::new(Buffer::new(1, file_name)?));
        let pane = Pane::new(1, buffer.clone(), lsp, theme, config);
        let window = Window::new(1, pane, lsp);
        Ok(Self {
            events: Events::new(config),
            view: View::new(lsp, config, theme, window)?,
            theme,
            config,
            lsp,
            mode: Mode::Normal,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.view.initialize()?;

        let mut stream = EventStream::new();
        let mut client = LspClient::start().await.unwrap();
        client.initialize().await?;

        loop {
            let delay = futures_timer::Delay::new(Duration::from_millis(300)).fuse();
            let event = stream.next().fuse();

            tokio::select! {
                _ = delay => {
                    if let Some((msg, _method)) = client.try_read_message().await? {
                        logger::trace!("[LSP] received message {msg:?}");
                    }
                }
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            if let Some(action) = self.events.handle(&event, &self.mode) {
                                match action {
                                    KeyAction::Single(Action::Quit) => {
                                        logger::trace!("user exiting session");
                                        self.view.handle(action)?;
                                        break
                                    }
                                    _ => self.view.handle(action)?,

                                }
                            }
                        }
                        Some(Err(_)) => (),
                        None => (),
                    }
                }
            }
        }

        Ok(())
    }
}
