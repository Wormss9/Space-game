use glium::{
    glutin::{
        event,
        event_loop::{ControlFlow, EventLoopWindowTarget},
    },
    Display, Surface,
};

use crate::clock::Clock;

pub fn event_handler(
    display: &Display,
    clock: &mut Clock,
    event: event::Event<()>,
    _event_loop_window_target: &EventLoopWindowTarget<()>,
    control_flow: &mut ControlFlow,
) {
    let (_delta_time, _time_squared) = clock.get_time();

    let mut target = display.draw();

    target.clear_color(0.5, 0.5, 0.0, 0.0);

    let _ = target.finish();

    match event {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        },
        _ => (),
    }
}
