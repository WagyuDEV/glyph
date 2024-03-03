mod event_handler;

use event_handler::TuiEventHandler;

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
    let lsp = glyph_core::LspClient::start().await?;
    let config = glyph_core::load_config()?;
    let theme = glyph_core::load_theme(
        &config.background,
        &config.theme,
        glyph_core::Config::themes_path(),
    )?;
    let event_handler = TuiEventHandler::new(&config);
    glyph_core::Editor::new(&config, &theme, lsp, file_name, event_handler).await?;
    Ok(())
}
