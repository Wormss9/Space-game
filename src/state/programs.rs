use glium::{
    program::{ProgramCreationInput, SourceCode},
    Display, Program,
};
use std::collections::HashMap;

pub struct Programs {
    programs: HashMap<ProgramName, Program>,
}

impl Programs {
    pub fn new(display: &Display) -> Self {
        let mut programs = HashMap::new();

        programs.insert(ProgramName::Hex, hex_program(display));

        Self { programs }
    }

    pub fn get(&self, name: &ProgramName) -> &Program {
        self.programs.get(name).unwrap()
    }
}

fn hex_program(display: &Display) -> Program {
    let vertex_shader_code = std::fs::read_to_string("shaders/base.vert").unwrap();
    let fragment_shader_code = std::fs::read_to_string("shaders/base.frag").unwrap();

    let program_creation_input_source = SourceCode {
        vertex_shader: &vertex_shader_code,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: &fragment_shader_code,
    };

    let program = ProgramCreationInput::from(program_creation_input_source);

    Program::new(display, program).unwrap()
}

#[derive(Eq, Hash, PartialEq)]
pub enum ProgramName {
    Hex,
}
