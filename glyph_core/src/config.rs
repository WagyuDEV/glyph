use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::editor::KeyAction;

const fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineNumbers {
    Absolute,
    Relative,
    RelativeNumbered,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EditorBackground {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde()]
    pub keys: Keys,
    pub theme: String,
    pub log_file: Option<String>,
    pub mouse_scroll_lines: Option<usize>,
    pub gutter_width: usize,
    pub line_numbers: LineNumbers,
    pub background: EditorBackground,
    pub empty_line_char: char,
    #[serde(default = "default_true")]
    pub show_diagnostics: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Keys {
    #[serde(default)]
    pub normal: HashMap<String, KeyAction>,
    #[serde(default)]
    pub insert: HashMap<String, KeyAction>,
    #[serde(default)]
    pub command: HashMap<String, KeyAction>,
}

impl Config {
    pub fn get_path() -> PathBuf {
        let home = dirs::home_dir().unwrap();
        home.join(".config/glyph")
    }

    pub fn themes_path() -> PathBuf {
        let config_path = Config::get_path();
        config_path.join("themes")
    }
}
