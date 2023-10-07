use glium::{
    glutin::{
        event,
        event_loop::{ControlFlow, EventLoopWindowTarget},
    },
    Display, Surface,
};

use crate::state::State;

pub fn run_systems(
    display: &Display,
    state: &mut State,
    event: event::Event<()>,
    _event_loop_window_target: &EventLoopWindowTarget<()>,
    control_flow: &mut ControlFlow,
) {
    let (_delta_time, _time_squared) = state.clock.get_time();

    let mut target = display.draw();

    target.clear_color(0.0, 0.0, 0.7, 0.0);

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
