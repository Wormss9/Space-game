use glium::{uniform, Blend, DrawParameters, Surface};
use hecs::Entity;

use crate::{
    components::{Floor, Ship},
    state::{ProgramName, State, VertexBufferName},
    vector::Vector,
};

pub fn render_system(state: &State) {
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut target = state.display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let draw_parameters = DrawParameters {
        blend: Blend::alpha_blending(),
        viewport: None,
        ..Default::default()
    };

    let mut ships = state.world.query::<&Ship>();
    let mut floors = state.world.query::<(&Entity, &Floor)>();
    for (_entity, ship) in ships.iter() {
        for (_, (_floor_entity, floor)) in floors.iter() {
            let uniforms = uniform! {
                tex: glium::uniforms::Sampler::new(state.textures.get_texture("floors", &floor.texture).unwrap())
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp),
                mov:adjust_a_r((ship.position+coords_to_pos(&floor.position, state.scale, ship.rotation)).position,state.aspect_ratio),
                sca:state.scale,
                rot:ship.rotation,
                rat:state.aspect_ratio
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
    target.finish().unwrap();
}

fn coords_to_pos(coord: &[i32; 2], scale: f32, rotaton: f32) -> Vector {
    //Hexagon height is 2

    let x_offset = 8.0 / 9.0 / 1.11; //Hexagon size ratio / texture scale
    let y_offset = 6.0 / 4.0 / 1.11; //Height * vertical spacing / texture scale

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
    [pos[0] / ar, pos[1]]
}
