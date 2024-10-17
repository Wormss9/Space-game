use super::settings::{FullscreenSetting, Settings};
use glium::{
    backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface,
    winit::event_loop::EventLoop, Display,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Window, WindowId},
};

impl FullscreenSetting {
    pub fn to_setting(&self, window: Window) -> Option<Fullscreen> {
        match self {
            FullscreenSetting::Exclusive => {
                let video_mode = window
                    .primary_monitor()
                    .unwrap()
                    .video_modes()
                    .min()
                    .unwrap();
                Some(Fullscreen::Exclusive(video_mode))
            }
            FullscreenSetting::Borderless => Some(Fullscreen::Borderless(window.primary_monitor())),
            FullscreenSetting::Windowed => None,
        }
    }
}

pub fn create_display(
    event_loop: &EventLoop<()>,
    settings: &Settings,
) -> (Window, Display<WindowSurface>) {
    SimpleWindowBuilder::new()
        .with_inner_size(settings.window_width, settings.window_height)
        .with_title("Space-game")
        .build(event_loop)
}
#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
