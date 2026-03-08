use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
struct ConfigFile {
    #[serde(default)]
    palette: Option<String>,
    #[serde(default)]
    symbols: Option<String>,
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("rem").join("config.toml"))
}

fn load_config() -> ConfigFile {
    if let Some(path) = config_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str::<ConfigFile>(&content) {
                return config;
            }
        }
    }
    ConfigFile::default()
}

pub fn load_palette_name() -> String {
    // CLI arg takes precedence
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--palette" {
            if let Some(name) = args.get(i + 1) {
                return name.clone();
            }
        }
        if let Some(name) = args[i].strip_prefix("--palette=") {
            return name.to_string();
        }
    }

    let config = load_config();
    config.palette.unwrap_or_else(|| "phosphor".to_string())
}

pub fn load_symbol_set_name() -> String {
    // CLI arg takes precedence
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--symbols" {
            if let Some(name) = args.get(i + 1) {
                return name.clone();
            }
        }
        if let Some(name) = args[i].strip_prefix("--symbols=") {
            return name.to_string();
        }
    }

    let config = load_config();
    config.symbols.unwrap_or_else(|| "standard".to_string())
}

pub fn save_config(palette: &str, symbols: &str) {
    if let Some(path) = config_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let config = ConfigFile {
            palette: Some(palette.to_string()),
            symbols: Some(symbols.to_string()),
        };
        if let Ok(content) = toml::to_string_pretty(&config) {
            let _ = fs::write(&path, content);
        }
    }
}
