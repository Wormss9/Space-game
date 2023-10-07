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
use window::create_display;

mod components;
mod settings;
mod state;
mod systems;
mod vertex;
mod window;

fn main() {
    let event_loop = EventLoop::new();
    let settings = Settings::get_config(&event_loop);
    let display = create_display(&event_loop, &settings);

    let mut state = State::new(&display);

    let control = PlayerControlled {};
    let ship = Ship {
        position: [0.0, 0.0],
        rotation: 0.0,
    };

    let ship_entity = state.world.spawn((control, ship));

    let floor = Floor {
        position: [0, 0],
        texture: "floor1".to_owned(),
    };

    state.world.spawn((ship_entity, floor));

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut x_pos = 0.5 as f32;
    let mut y_pos = 0.5;

    let mut scale = 0.5 as f32;
    let mut rotation = 0.0;

    event_loop.run(move |ev, _, control_flow| {
        let ratio = 9.0 / 16.0 as f32;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            viewport: None,
            ..Default::default()
        };

        let mut ships = state.world.query::<(&Ship)>();
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
                    mov:ship.position,
                    sca:state.scale,
                    rot:ship.rotation,
                    rat:ratio
                };

                target
                .draw(
                    state.vertex_buffers.get(&VertexBufferName::Hex),
                    &indices,
                    state.programs.get(&ProgramName::Hex),
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
            }
        }

        target.finish().unwrap();

        match ev {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                event::WindowEvent::KeyboardInput { input, .. } => {
                    if let event::ElementState::Pressed = input.state {
                        match input.virtual_keycode {
                            Some(event::VirtualKeyCode::A) => {
                                x_pos = x_pos - 0.05;
                            }
                            Some(event::VirtualKeyCode::D) => {
                                x_pos = x_pos + 0.05;
                            }
                            Some(event::VirtualKeyCode::W) => {
                                y_pos = y_pos + 0.05;
                            }
                            Some(event::VirtualKeyCode::S) => {
                                y_pos = y_pos - 0.05;
                            }
                            Some(event::VirtualKeyCode::LControl) => {
                                scale = scale - 0.05;
                            }
                            Some(event::VirtualKeyCode::LShift) => {
                                scale = scale + 0.05;
                            }
                            Some(event::VirtualKeyCode::Q) => {
                                rotation = rotation - 0.05;
                                if rotation < 0.0 {
                                    rotation += std::f32::consts::PI;
                                }
                            }
                            Some(event::VirtualKeyCode::E) => {
                                rotation = rotation + 0.05;
                                if rotation > std::f32::consts::PI {
                                    rotation -= std::f32::consts::PI;
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
