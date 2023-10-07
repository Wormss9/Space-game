use crate::{
    components::{PlayerControlled, Ship},
    state::State,
};
use glium::glutin::{dpi::PhysicalSize, event, event_loop::ControlFlow};

pub fn controls_system(state: &mut State, ev: event::Event<()>, control_flow: &mut ControlFlow) {
    let mut controlled_ship: Option<&mut Ship> = None;

    let ships = state.world.query_mut::<(&PlayerControlled, &mut Ship)>();
    for (_, (_, ship)) in ships.into_iter() {
        controlled_ship = Some(ship)
    }
    let controlled_ship = controlled_ship.unwrap();

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
                    match input.virtual_keycode {
                        Some(event::VirtualKeyCode::A) => {
                            controlled_ship.position.position[0] -= 0.05;
                        }
                        Some(event::VirtualKeyCode::D) => {
                            controlled_ship.position.position[0] += 0.05;
                        }
                        Some(event::VirtualKeyCode::W) => {
                            controlled_ship.position.position[1] += 0.05;
                        }
                        Some(event::VirtualKeyCode::S) => {
                            controlled_ship.position.position[1] -= 0.05;
                        }
                        Some(event::VirtualKeyCode::LControl) => {
                            state.scale -= 0.05;
                        }
                        Some(event::VirtualKeyCode::LShift) => {
                            state.scale += 0.05;
                        }
                        Some(event::VirtualKeyCode::Q) => {
                            controlled_ship.rotation -= 0.05;
                            if controlled_ship.rotation < 0.0 {
                                controlled_ship.rotation += std::f32::consts::PI * 2.0;
                            }
                        }
                        Some(event::VirtualKeyCode::E) => {
                            controlled_ship.rotation += 0.05;
                            if controlled_ship.rotation > std::f32::consts::PI * 2.0 {
                                controlled_ship.rotation -= std::f32::consts::PI * 2.0;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }
}
