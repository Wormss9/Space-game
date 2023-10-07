use crate::state::State;
use glium::glutin::{
    event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use self::{controls::controls_system, render::render_system};

mod controls;
mod render;

pub fn run_systems(
    state: &mut State,
    event: event::Event<()>,
    _event_loop_window_target: &EventLoopWindowTarget<()>,
    control_flow: &mut ControlFlow,
) {
    let (_delta_time, _time_squared) = state.clock.get_time();

    render_system(state);
    controls_system(state, event, control_flow);
}
