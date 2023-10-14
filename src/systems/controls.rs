use crate::state::{settings::keybindings::Actions, State};
use glium::glutin::{dpi::PhysicalSize, event, event_loop::ControlFlow};

pub fn controls_system(state: &mut State, ev: event::Event<()>, control_flow: &mut ControlFlow) {
    match ev {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::Resized(PhysicalSize { width, height }) => {
                state.aspect_ratio = width as f32 / height as f32;
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
        // Actions::MoveUp => controlled_ship.position.position[1] += 0.05,
        // Actions::MoveDown => controlled_ship.position.position[1] -= 0.05,
        // Actions::MoveLeft => controlled_ship.position.position[0] -= 0.05,
        // Actions::MoveRight => controlled_ship.position.position[0] += 0.05,
        // Actions::RotateClockwise => {
        //     controlled_ship.rotation += 0.05;
        //     if controlled_ship.rotation > std::f32::consts::PI * 2.0 {
        //         controlled_ship.rotation -= std::f32::consts::PI * 2.0;
        //     }
        // }
        // Actions::RotateCounterClockwise => {
        //     controlled_ship.rotation -= 0.05;
        //     if controlled_ship.rotation < 0.0 {
        //         controlled_ship.rotation += std::f32::consts::PI * 2.0;
        //     }
        // }
        Actions::ZoomIn => state.scale *= 1.05,
        Actions::ZoomOut => state.scale /= 1.05,
        _ => (),
    }
}
