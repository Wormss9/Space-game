use glium::glutin::event_loop::EventLoop;
use state::State;

mod components;
mod state;
mod systems;
mod vector;
mod vertex;

fn main() {
    let event_loop = EventLoop::new();
    let mut state = State::new(&event_loop);
    state.add_test_ship();

    event_loop.run(move |event, event_loop_window_target, control_flow| {
        systems::run_systems(&mut state, event, event_loop_window_target, control_flow);
    });
}
