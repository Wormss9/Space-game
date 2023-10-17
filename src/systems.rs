use crate::state::State;
use glium::glutin::{
    event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use self::{
    controls::controls_system, gravity::gravity_system, movement::movement_system,
    render::render_system,
};

mod controls;
mod gravity;
mod movement;
mod render;

pub fn run_systems(
    state: &mut State,
    event: event::Event<()>,
    _event_loop_window_target: &EventLoopWindowTarget<()>,
    control_flow: &mut ControlFlow,
) {
    let delta_time_data = state.clock.get_time();

    state.camera.update(&state.world);

    gravity_system(state);
    movement_system(state, delta_time_data);
    render_system(state);
    controls_system(state, event, control_flow);
}
