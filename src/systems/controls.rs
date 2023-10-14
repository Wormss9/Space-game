use crate::state::{settings::keybindings::Actions, State};
use glium::glutin::{dpi::PhysicalSize, event, event_loop::ControlFlow};

pub fn controls_system(state: &mut State, ev: event::Event<()>, control_flow: &mut ControlFlow) {
    match ev {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::Resized(PhysicalSize { width, height }) => {
                state.camera.aspect_ratio = width as f64 / height as f64;
            }
            event::WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            event::WindowEvent::KeyboardInput { input, .. } => {
                if let event::ElementState::Pressed = input.state {
                    handle_input(state, input.virtual_keycode)
                }
            }
            _ => (),
        },
        _ => (),
    }
}

fn handle_input(state: &mut State, input: Option<event::VirtualKeyCode>) {
    if let Some(p_key) = input {
        for binding in state.settings.keybinds.iter() {
            if p_key == binding.key {
                perform_action(binding.action, state);
                break;
            }
        }
    }
}

fn perform_action(action: Actions, state: &mut State) {
    match action {
        Actions::SaveShip => todo!(),
        Actions::LoadShip => todo!(),
        Actions::MoveUp => state.camera.position.position[1] += state.camera.scale,
        Actions::MoveDown => state.camera.position.position[1] -= state.camera.scale,
        Actions::MoveLeft => state.camera.position.position[0] -= state.camera.scale,
        Actions::MoveRight => state.camera.position.position[0] += state.camera.scale,
        Actions::RotateClockwise => {
            state.camera.rotation += 0.05;
            if state.camera.rotation > std::f64::consts::PI * 2.0 {
                state.camera.rotation -= std::f64::consts::PI * 2.0;
            }
        }
        Actions::RotateCounterClockwise => {
            state.camera.rotation -= 0.05;
            if state.camera.rotation < 0.0 {
                state.camera.rotation += std::f64::consts::PI * 2.0;
            }
        }
        Actions::ZoomIn => state.camera.scale *= 1.05,
        Actions::ZoomOut => state.camera.scale /= 1.05,
        _ => (),
    }
    println!("{:?}", state.camera)
}
