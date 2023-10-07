use glium::Display;
use hecs::World;

pub use self::{
    clock::Clock,
    game_state::GameState,
    programs::{ProgramName, Programs},
    textures::Textures,
    vertex_buffers::{VertexBufferName, VertexBuffers},
};

mod clock;
mod game_state;
mod programs;
mod textures;
mod vertex_buffers;

pub struct State {
    // pub calendar: GameCalendar,
    pub clock: Clock,
    pub world: World,
    pub textures: Textures,
    // pub sounds: HashMap<String, glium::AudioBuffer>,
    pub game_state: GameState,
    pub programs: Programs,
    pub vertex_buffers: VertexBuffers,
    pub scale: f32,
    pub aspect_ratio: f64,
}

impl State {
    pub fn new(display: &Display) -> Self {
        Self {
            clock: Clock::new(),
            world: World::new(),
            textures: Textures::new(display),
            game_state: GameState::MainMenu,
            programs: Programs::new(display),
            vertex_buffers: VertexBuffers::new(display),
            scale: 1.0,
            aspect_ratio: 1.0,
        }
    }
}
