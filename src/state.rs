pub use self::{
    camera::Camera,
    clock::Clock,
    game_state::GameState,
    programs::{ProgramName, Programs},
    settings::Settings,
    textures::Textures,
    vertex_buffers::{VertexBufferName, VertexBuffers},
    window::create_display
};
use crate::{
    components::{Position, Rotation, Ship, ShipParts, Tile},
    vector::Vector,
};
use glium::{glutin::surface::WindowSurface, Display,winit::event_loop::EventLoop};
use hecs::World;
use winit::window::Window;

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
    pub display: Display<WindowSurface>,
    pub clock: Clock,
    pub textures: Textures,
    pub game_state: GameState,
    pub programs: Programs,
    pub vertex_buffers: VertexBuffers,
    pub camera: Camera,
    pub world: World,
    pub window: Window,
}

impl State {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let settings = Settings::get_config();
        let (window, display) = create_display(event_loop, &settings);

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
            window,
        }
    }
    pub fn add_test_ship(&mut self) {
        let mut ship = Ship {
            parts: vec![
                ShipParts::Floor(Tile {
                    position: [0, 0],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [-1, 0],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [-1, 0],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [0, 1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [0, 1],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [-1, -1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [-1, -1],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [1, 1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [1, 1],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [0, -1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [1, 0],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [-1, -2],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [-1, -2],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [2, 1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [2, 1],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [0, -2],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [0, -2],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [1, -1],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [1, -1],
                    texture: "wall1".to_owned(),
                }),
                ShipParts::Floor(Tile {
                    position: [2, 0],
                    texture: "floor1".to_owned(),
                }),
                ShipParts::Wall(Tile {
                    position: [2, 0],
                    texture: "wall1".to_owned(),
                }),
            ],
        };
        ship.sort_parts();
        let position = Position {
            position: Vector::new(0.0, 0.0),
            acceleration: Vector::new(0.0, 0.0),
            speed: Vector::new(0.0, 0.0),
        };
        let rotation = Rotation {
            rotation: 0.0,
            rotation_speed: 1.0,
        };

        self.world.spawn((ship, position, rotation));
    }
}
