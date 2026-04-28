use dirs::config_dir;
use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize)]
pub struct Config {
    pub album_art: bool,
}

fn config_path() -> Option<std::path::PathBuf> {
    config_dir().map(|p| p.join("music_info/config.toml"))
}

pub fn load_config() -> Config {
    if let Some(path) = config_path() {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|c| from_str(&c).ok())
            .unwrap_or(Config { album_art: true })
    } else {
        Config { album_art: true }
    }
}
