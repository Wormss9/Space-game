use glium::{
    glutin::{event_loop::EventLoop, window::{Fullscreen, WindowBuilder}, ContextBuilder, dpi::LogicalSize},
    Display,
};

use crate::settings::{FullscreenSetting, Settings};

impl FullscreenSetting {
    pub fn to_setting(&self, event_loop: &EventLoop<()>) -> Option<Fullscreen> {
        match self {
            FullscreenSetting::Exclusive => {
                let video_mode = event_loop
                    .primary_monitor()
                    .unwrap()
                    .video_modes()
                    .min()
                    .unwrap();
                Some(Fullscreen::Exclusive(video_mode))
            }
            FullscreenSetting::Borderless => {
                Some(Fullscreen::Borderless(event_loop.primary_monitor()))
            }
            FullscreenSetting::Windowed => None,
        }
    }
}

pub fn create_display(event_loop: &EventLoop<()>, settings: &Settings) -> Display {
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(
            settings.window_width,
            settings.window_height,
        ))
        .with_fullscreen(settings.fullscreen.to_setting(event_loop))
        .with_title("Space-game");
    let cb = ContextBuilder::new();
    Display::new(wb, cb, event_loop).unwrap()
}
