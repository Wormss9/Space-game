use glium::{uniform, Blend, DrawParameters, Surface};

use crate::state::{ProgramName, State, VertexBufferName};

pub fn render_system(state: &State) {
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut target = state.display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let draw_parameters = DrawParameters {
        blend: Blend::alpha_blending(),
        viewport: None,
        ..Default::default()
    };

    for ship in state.ships.iter() {
        for part in ship.parts.iter() {
            let (position, texture_type, texture_name) = match part {
                crate::entities::ship::ShipParts::Floor(floor) => {
                    (floor.position, "floors", &floor.texture)
                }
            };

            let uniforms = uniform! {
                tex: glium::uniforms::Sampler::new(state.textures.get_texture(texture_type, texture_name).unwrap())
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp),
                mov:ship.position.adjust_position(&position, state.scale, ship.rotation).adjust_ratio(state.aspect_ratio).position,
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
