mod commandline;
mod editor;
mod event_handler;
mod statusline;

use commandline::TuiCommandline;
use glyph_core::config::Config;
use glyph_core::editor::{Commandline, Size, Statusline};
use glyph_core::lsp::LspClient;

use editor::{EditorSetup, TuiEditor};
use event_handler::TuiEventHandler;

use glyph_core::window::Rect;
use statusline::TuiStatusline;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let appender = tracing_appender::rolling::never(".", "glyph.log");
    let (writer, _guard) = tracing_appender::non_blocking(appender);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(writer)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let file_name = std::env::args().nth(1);
    let lsp = LspClient::start().await?;
    let config = glyph_core::load_config()?;
    let theme = glyph_core::load_theme(&config.background, &config.theme, Config::themes_path())?;
    let event_handler = TuiEventHandler::new(&config);
    let size: Size = crossterm::terminal::size()?.into();
    let statusline = TuiStatusline::new(Rect::new(0, size.height - 2, size.width, 1), &theme);
    let commandline = TuiCommandline::new(Rect::new(0, size.height - 1, size.width, 1));
    let editor_setup = EditorSetup {
        config: &config,
        theme: &theme,
        lsp,
        file_name,
        size,
    };
    let mut editor = TuiEditor::new(editor_setup, statusline, commandline, event_handler)?;
    editor.start().await?;

    Ok(())
}
