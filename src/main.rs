#![allow(dead_code)]

use glium::winit::event_loop::EventLoop;
use state::State;

mod components;
mod state;
mod systems;
mod vector;
mod vertex;

fn main() {
    let event_loop = EventLoop::new().expect("Faild to build event loop");
    let mut state = State::new(&event_loop);

    state.add_test_ship();

    let _ = event_loop.run(move |event, _| {
        systems::run_systems(&mut state, event);
    });
}
