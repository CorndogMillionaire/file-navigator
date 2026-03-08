use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Default)]
struct ConfigFile {
    #[serde(default)]
    palette: Option<String>,
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("rem").join("config.toml"))
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

    // Then config file
    if let Some(path) = config_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str::<ConfigFile>(&content) {
                if let Some(name) = config.palette {
                    return name;
                }
            }
        }
    }

    "phosphor".to_string()
}
