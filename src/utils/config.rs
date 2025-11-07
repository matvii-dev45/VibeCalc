use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub background_color: String,
    pub button_color: String,
    pub button_hover_color: String,
    pub button_pressed_color: String,
    pub font_size: u32,
    pub font_path: Option<String>,
}

///
/// Reads the configuration from "config.toml".
///
pub fn read_config() -> Result<Config> {
    let config_str = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

///
/// Creates a default configuration.
///
pub fn create_default_config() -> Config {
    Config {
        window_width: 400,
        window_height: 600,
        background_color: "#282c34".to_string(),
        button_color: "#61afef".to_string(),
        button_hover_color: "#528bbd".to_string(),
        button_pressed_color: "#3a6ea5".to_string(),
        font_size: 18,
        font_path: None,
    }
}

///
/// Saves the default configuration to "config.toml".
///
pub fn save_default_config() -> anyhow::Result<()> {
    let config = create_default_config();
    let toml_str = toml::to_string_pretty(&config)?;
    std::fs::write("config.toml", toml_str)?;
    Ok(())
}
