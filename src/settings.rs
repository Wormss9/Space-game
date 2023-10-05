use glium::glutin::event_loop::EventLoop;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: FullscreenSetting,
}

#[derive(Debug, Deserialize)]
pub enum FullscreenSetting {
    Exclusive,
    Borderless,
    Windowed,
}

pub enum ConfigError {
    FileError(std::io::Error),
    ParseError(serde_json::Error),
}

impl Settings {
    fn load_config() -> Result<Self, ConfigError> {
        let mut file = File::open("config.json").map_err(ConfigError::FileError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(ConfigError::FileError)?;
        serde_json::from_str(&contents).map_err(ConfigError::ParseError)
    }
    fn create_config(event_loop: &EventLoop<()>) -> Self {
        let (window_width, window_height) = match event_loop.primary_monitor() {
            Some(monitor) => {
                let size = monitor.size();
                (size.width, size.height)
            }
            None => (800, 600),
        };
        Self {
            window_width,
            window_height,
            fullscreen: FullscreenSetting::Exclusive,
        }
    }
    pub fn get_config(event_loop: &EventLoop<()>) -> Self {
        match Self::load_config() {
            Ok(config) => config,
            Err(_) => Self::create_config(event_loop),
        }
    }
}
