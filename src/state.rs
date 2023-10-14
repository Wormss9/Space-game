pub use self::{
    camera::Camera,
    clock::Clock,
    game_state::GameState,
    programs::{ProgramName, Programs},
    settings::Settings,
    textures::Textures,
    vertex_buffers::{VertexBufferName, VertexBuffers},
    window::create_display,
};
use crate::{
    components::{Floor, GravityMass, Planet, Position, Rotation, Ship, ShipParts},
    vector::Vector,
};
use glium::{glutin::event_loop::EventLoop, Display};
use hecs::World;

mod camera;
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
    pub camera: Camera,
    pub world: World,
}

impl State {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let settings = Settings::get_config(event_loop);
        let display = create_display(event_loop, &settings);

        Self {
            settings,
            clock: Clock::new(),
            textures: Textures::new(&display),
            game_state: GameState::MainMenu,
            programs: Programs::new(&display),
            vertex_buffers: VertexBuffers::new(&display),
            camera: Camera::new(&display),
            display,
            world: World::new(),
        }
    }
    pub fn add_test_ship(&mut self) {
        let ship = Ship {
            parts: vec![ShipParts::Floor(Floor {
                position: [0, 0],
                texture: "floor1".to_owned(),
            })],
        };
        let position = Position {
            position: Vector::new(0.0, -2.0),
            acceleration: Vector::new(0.0, 0.0),
            speed: Vector::new(-1.5, 0.0),
            debug: 0.0,
        };
        let rotation = Rotation {
            rotation: 0.0,
            rotation_speed: 0.0,
        };

        self.world.spawn((ship, position, rotation));

        let ship = Ship {
            parts: vec![ShipParts::Floor(Floor {
                position: [0, 0],
                texture: "floor1".to_owned(),
            })],
        };
        let position = Position {
            position: Vector::new(0.0, 3.0),
            acceleration: Vector::new(0.0, 0.0),
            speed: Vector::new(2.0, 0.0),
            debug: 0.0,
        };
        let rotation = Rotation {
            rotation: 0.0,
            rotation_speed: 0.0,
        };

        self.world.spawn((ship, position, rotation));

        let mass = GravityMass { mass: 10.0 };
        let planet = Planet { radius: 0.5 };
        let position = Position {
            position: Vector::new(0.0, 0.0),
            acceleration: Vector::new(0.0, 0.0),
            speed: Vector::new(0.0, 0.0),
            debug: 0.0,
        };

        self.world.spawn((mass, planet, position));
    }
}
