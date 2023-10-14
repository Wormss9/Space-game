pub use self::{
    clock::Clock,
    game_state::GameState,
    programs::{ProgramName, Programs},
    settings::Settings,
    textures::Textures,
    vertex_buffers::{VertexBufferName, VertexBuffers},
    window::create_display,
};
use crate::{
    entities::{
        celestial_body::CelestialBodies,
        ship::{Floor, Ship, ShipParts},
    },
    vector::Vector,
};
use glium::{glutin::event_loop::EventLoop, Display};

mod clock;
mod game_state;
mod programs;
pub mod settings;
mod textures;
mod vertex_buffers;
mod window;

pub struct State {
    pub settings: Settings,
    pub display: Display,
    pub clock: Clock,
    pub textures: Textures,
    pub game_state: GameState,
    pub programs: Programs,
    pub vertex_buffers: VertexBuffers,
    pub scale: f32,
    pub aspect_ratio: f32,
    pub celestial_bodies: Vec<CelestialBodies>,
    pub ships: Vec<Ship>,
}

impl State {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let settings = Settings::get_config(event_loop);
        let display = create_display(event_loop, &settings);
        let (x, y) = display.get_framebuffer_dimensions();
        let aspect_ratio = x as f32 / y as f32;

        Self {
            settings,
            clock: Clock::new(),
            textures: Textures::new(&display),
            game_state: GameState::MainMenu,
            programs: Programs::new(&display),
            vertex_buffers: VertexBuffers::new(&display),
            scale: 0.3,
            aspect_ratio,
            display,
            celestial_bodies: Vec::new(),
            ships: Vec::new(),
        }
    }
    pub fn add_test_ship(&mut self) {
        let ship = Ship {
            position: Vector {
                position: [0.0, 0.0],
            },
            rotation: 0.0,
            parts: vec![
                ShipParts::Floor(Floor {
                    position: [0, 0],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Floor {
                    position: [-1, 0],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Floor {
                    position: [0, 1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Floor {
                    position: [-1, 1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Floor {
                    position: [-2, 0],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Floor {
                    position: [0, 2],
                    texture: "floor1".to_owned(),
                }),
            ],
            speed: Vector {
                position: [0.0, 0.0],
            },
            rotation_speed: 0.0,
        };
        self.ships.push(ship);
    }
}
