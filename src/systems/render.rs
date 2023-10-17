use glium::{uniform, Blend, DrawParameters, Surface};

use crate::{
    components::{Position, Rotation, Ship, ShipParts},
    state::{ProgramName, State, VertexBufferName},
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

    for (_, (ship, position, rotation)) in
        state.world.query::<(&Ship, &Position, &Rotation)>().iter()
    {
        for part in ship.parts.iter() {
            let (part_position, texture_type, texture_name) = match part {
                ShipParts::Floor(floor) => (floor.position, "floors", &floor.texture),
            };

            let scale = state.camera.scale;
            let rotation = rotation.rotation + state.camera.rotation;
            let aspect_ratio = state.camera.aspect_ratio;

            let uniforms = uniform! {
                tex: glium::uniforms::Sampler::new(state.textures.get_texture(texture_type, texture_name).unwrap())
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp),
                mov:(((position.position-state.camera.focus_object_position)*scale).adjust_position(&part_position, state.camera.scale, rotation).adjust_ratio(state.camera.aspect_ratio) + state.camera.position).as_f32(),
                sca:scale as f32,
                rot:rotation as f32,
                rat:aspect_ratio as f32
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
