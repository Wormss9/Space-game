use glium::{
    glutin::{
        event,
        event_loop::{ControlFlow, EventLoop},
    },
    program::{ProgramCreationInput, SourceCode},
    Program, Surface,
};
use settings::Settings;
use state::State;
use vertex::Vertex;
use window::create_display;

mod event_handler;
mod settings;
mod state;
mod vertex;
mod window;

fn main() {
    let event_loop = EventLoop::new();
    let settings = Settings::get_config(&event_loop);
    let display = create_display(&event_loop, &settings);

    // Read the vertex shader code from a file.
    let vertex_shader_code = std::fs::read_to_string("assets/shaders/test.vert").unwrap();

    // Read the fragment shader code from a file.
    let fragment_shader_code = std::fs::read_to_string("assets/shaders/test.frag").unwrap();

    // Create a program creation input from the vertex shader code and the fragment shader code.
    let program_creation_input_source = SourceCode {
        vertex_shader: &vertex_shader_code,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: &fragment_shader_code,
    };

    let program = ProgramCreationInput::from(program_creation_input_source);

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    // draw the triangle here
    // Create a shader program.
    let program = Program::new(&display, program).unwrap();

    let mut state = State::new();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();

    event_loop.run(move |ev, _, control_flow| match ev {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        },
        _ => (),
    });

    // event_loop.run(move |event, event_loop_window_target, control_flow| {
    //     event_handler::event_handler(
    //         &display,
    //         &mut state,
    //         event,
    //         event_loop_window_target,
    //         control_flow,
    //     );
    // });
}
