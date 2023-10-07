use components::{Floor, PlayerControlled, Ship};
use glium::{
    glutin::{
        event,
        event_loop::{ControlFlow, EventLoop},
    },
    uniform, Blend, DrawParameters, Surface,
};
use hecs::Entity;
use settings::Settings;
use state::{ProgramName, State, VertexBufferName};
use vector::Vector;
use window::create_display;

mod components;
mod settings;
mod state;
mod systems;
mod vector;
mod vertex;
mod window;

fn main() {
    let event_loop = EventLoop::new();
    let settings = Settings::get_config(&event_loop);
    let display = create_display(&event_loop, &settings);

    let mut state = State::new(&display);

    let control = PlayerControlled {};
    let ship = Ship {
        position: Vector {
            position: [0.0, 0.0],
        },
        rotation: 0.0,
    };

    let ship_entity = state.world.spawn((control, ship));

    let floor1 = Floor {
        position: [0, 0],
        texture: "floor1".to_owned(),
    };

    let floor2 = Floor {
        position: [-1, 0],
        texture: "floor1".to_owned(),
    };

    let floor3 = Floor {
        position: [0, 1],
        texture: "floor1".to_owned(),
    };

    let floor4 = Floor {
        position: [-1, 1],
        texture: "floor1".to_owned(),
    };

    state.world.spawn((ship_entity, floor1));
    state.world.spawn((ship_entity, floor2));
    state.world.spawn((ship_entity, floor3));
    state.world.spawn((ship_entity, floor4));

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    event_loop.run(move |ev, _, control_flow| {
        let ratio = 9.0 / 16.0 as f32;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            viewport: None,
            ..Default::default()
        };

        {
        let mut ships = state.world.query::<&Ship>();
        let mut floors = state.world.query::<(&Entity,&Floor)>();
        for (entity, ship) in ships.iter() {
            println!("{:?} {:?}", entity, ship);
            for (_,(floor_entity,floor)) in floors.iter(){
                println!("{:?} {:?}", floor_entity, floor);
                let uniforms = uniform! {
                    tex: glium::uniforms::Sampler::new(state.textures.get_texture("floors", &floor.texture).unwrap())
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp),
                    mov:adjust_a_r((ship.position+coords_to_pos(&floor.position, state.scale, ship.rotation)).position,9.0/16.0),
                    sca:state.scale,
                    rot:ship.rotation,
                    rat:ratio
                };

                target
                .draw(
                    state.vertex_buffers.get(&VertexBufferName::Hex),
                    indices,
                    state.programs.get(&ProgramName::Hex),
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
            }
        }
    }
        target.finish().unwrap();

        let mut controlled_ship:Option<&mut Ship>=None;

        let ships = state.world.query_mut::<(&PlayerControlled,&mut Ship)>();
        for (_,(_,ship)) in ships.into_iter(){
            controlled_ship = Some(ship)
        }
        let controlled_ship = controlled_ship.unwrap();

        match ev {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                event::WindowEvent::KeyboardInput { input, .. } => {
                    if let event::ElementState::Pressed = input.state {
                        match input.virtual_keycode {
                            Some(event::VirtualKeyCode::A) => {
                                controlled_ship.position.position[0] -=0.05;
                            }
                            Some(event::VirtualKeyCode::D) => {
                                controlled_ship.position.position[0] +=0.05;
                            }
                            Some(event::VirtualKeyCode::W) => {
                                controlled_ship.position.position[1] +=0.05;
                            }
                            Some(event::VirtualKeyCode::S) => {
                                controlled_ship.position.position[1] -=0.05;
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
                                    controlled_ship.rotation += std::f32::consts::PI*2.0;
                                }
                            }
                            Some(event::VirtualKeyCode::E) => {
                                controlled_ship.rotation += 0.05;
                                if controlled_ship.rotation > std::f32::consts::PI*2.0 {
                                    controlled_ship.rotation -= std::f32::consts::PI*2.0;
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
    });

    // event_loop.run(move |event, event_loop_window_target, control_flow| {
    //     systems::run_systems(
    //         &display,
    //         &mut state,
    //         event,
    //         event_loop_window_target,
    //         control_flow,
    //     );
    // });
}

fn coords_to_pos(coord: &[i32; 2], scale: f32, rotaton: f32) -> Vector {
    let x_offset = 1.0 / 1.11 / 2.05 / 9.0 * 16.0;
    let y_offset = 3.0 / 4.0 * 1.87;

    let ur = Vector {
        position: [x_offset, y_offset],
    } * coord[0] as f32;

    let dr = Vector {
        position: [x_offset, -y_offset],
    } * coord[1] as f32;

    let pos = (ur + dr) * scale;

    let rot = [
        [rotaton.cos(), rotaton.sin()],
        [-rotaton.sin(), rotaton.cos()],
    ];

    pos * rot
}

fn adjust_a_r(pos: [f32; 2], ar: f32) -> [f32; 2] {
    [pos[0] * ar, pos[1]]
}
