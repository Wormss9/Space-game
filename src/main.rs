use clock::Clock;
use glium::glutin::event_loop::EventLoop;
use settings::Settings;
use window::create_display;

mod clock;
mod ecs;
mod event_handler;
mod settings;
mod state;
mod vertex;
mod window;

fn main() {
    let event_loop = EventLoop::new();
    let settings = Settings::get_config(&event_loop);
    let display = create_display(&event_loop, &settings);

    let mut clock = Clock::new();

    event_loop.run(move |event, event_loop_window_target, control_flow| {
        event_handler::event_handler(
            &display,
            &mut clock,
            event,
            event_loop_window_target,
            control_flow,
        );
    });
}
