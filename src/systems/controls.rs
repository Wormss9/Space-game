use winit::{
    dpi::PhysicalSize,
    event,
    keyboard::Key,
};

use crate::state::{settings::keybindings::Actions, State};

// pub fn controls_system(state: &mut State, ev: WindowEvent) {
//     match ev {
//         event::WindowEvent::Resized(PhysicalSize { width, height }) => {
//             state.camera.aspect_ratio = width as f64 / height as f64;
//         }
//         event::WindowEvent::KeyboardInput { event, .. } => handle_input(state, event.logical_key),
//         _ => (),
//     }
// }
pub fn controls_system(state: &mut State, ev: event::Event<()>) {
    match ev {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::Resized(PhysicalSize { width, height }) => {
                state.camera.aspect_ratio = width as f64 / height as f64;
            }
            event::WindowEvent::CloseRequested => {}
            event::WindowEvent::KeyboardInput { event, .. } => {
                handle_input(state, event.logical_key)
            }
            _ => (),
        },
        _ => (),
    }
}

fn handle_input(state: &mut State, input: Key) {
    for binding in state.settings.keybinds.iter() {
        if input == binding.key {
            println!("{:?}",binding.key);
            perform_action(binding.action, state);
            break;
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
    }
}
