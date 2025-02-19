pub mod buffer;
pub mod commandline;
pub mod config;
pub mod cursor;
pub mod editor;
pub mod event_handler;
pub mod gutter;
pub mod highlight;
pub mod lsp;
pub mod statusline;
pub mod tab;
pub mod theme;
pub mod ui;
pub mod window;

use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use config::{Config, EditorBackground};
use theme::{loader::ThemeLoader, Theme};

pub fn load_config() -> anyhow::Result<Config> {
    let config_dir = Config::get_path();
    let config_file = config_dir.join("glyph.toml");
    let config_file = Path::new(&config_file);
    // TODO: in the future, the initial config should be installed automatically
    if !config_file.exists() {
        tracing::error!("loaded failed");
        return Err(anyhow::Error::new(Error::new(
            ErrorKind::NotFound,
            "config file not found. please refer to the configuration section of the readme",
        )));
    }
    let toml = std::fs::read_to_string(config_file)?;
    tracing::error!("loaded config");
    let config: Config = toml::from_str(&toml)?;
    Ok(config)
}

pub fn load_theme(
    background: &EditorBackground,
    theme_name: &str,
    themes_dir: PathBuf,
) -> anyhow::Result<Theme> {
    if !themes_dir.exists() {
        std::fs::create_dir(&themes_dir)?;
        // TODO: install themes when first loading
    }
    let default = match background {
        EditorBackground::Light => Theme::light()?,
        EditorBackground::Dark => Theme::dark()?,
    };
    if theme_name.is_empty() {
        return Ok(default);
    }
    let theme_path = themes_dir.join(theme_name);
    match theme_path.exists() {
        false => Ok(default),
        true => {
            let toml = std::fs::read_to_string(theme_path)?;
            let theme: ThemeLoader = toml::from_str(&toml)?;
            Ok(theme.into())
        }
    }
}
