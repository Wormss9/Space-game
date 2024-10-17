use self::keybindings::{get_default_keybindings, KeyBinding};
use serde::Deserialize;
use winit::window::Window;
use std::fs::File;
use std::io::Read;

pub mod keybindings;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: FullscreenSetting,
    pub keybinds: Vec<KeyBinding>,
}

#[derive(Debug, Deserialize)]
pub enum FullscreenSetting {
    Exclusive,
    Borderless,
    Windowed,
}

pub enum ConfigError {
    FileError(std::io::Error),
    ParseError(ron::error::SpannedError),
}

impl Settings {
    fn load_config() -> Result<Self, ConfigError> {
        let mut file = File::open("config.json").map_err(ConfigError::FileError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(ConfigError::FileError)?;
        ron::from_str(&contents).map_err(ConfigError::ParseError)
    }
    fn create_config() -> Self {
        let (window_width, window_height) = (800, 600);
        Self {
            window_width,
            window_height,
            fullscreen: FullscreenSetting::Borderless,
            keybinds: get_default_keybindings(),
        }
    }
    pub fn get_config() -> Self {
        match Self::load_config() {
            Ok(config) => config,
            Err(_) => Self::create_config(),
        }
    }
}
