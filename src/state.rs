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
    components::{Floor, PlayerControlled, Ship},
    vector::Vector,
};
use glium::{glutin::event_loop::EventLoop, Display};
use hecs::World;

mod clock;
mod game_state;
mod programs;
mod settings;
mod textures;
mod vertex_buffers;
mod window;

pub struct State {
    pub settings: Settings,
    pub display: Display,
    pub clock: Clock,
    pub world: World,
    pub textures: Textures,
    pub game_state: GameState,
    pub programs: Programs,
    pub vertex_buffers: VertexBuffers,
    pub scale: f32,
    pub aspect_ratio: f32,
    // pub calendar: GameCalendar,
    // pub sounds: HashMap<String, glium::AudioBuffer>,
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
            world: World::new(),
            textures: Textures::new(&display),
            game_state: GameState::MainMenu,
            programs: Programs::new(&display),
            vertex_buffers: VertexBuffers::new(&display),
            scale: 1.0,
            aspect_ratio,
            display,
        }
    }

    pub fn add_test_ship(&mut self) {
        let control = PlayerControlled {};
        let ship = Ship {
            position: Vector {
                position: [0.0, 0.0],
            },
            rotation: 0.0,
        };

        let ship_entity = self.world.spawn((control, ship));

        let floor1 = Floor {
            position: [0, 0],
            texture: "floor1".to_owned(),
        };

        let floor2 = Floor {
            position: [-1, 0],
            texture: "floor1".to_owned(),
        };

        let floor3 = Floor {
            position: [0, 1],
            texture: "floor1".to_owned(),
        };

        let floor4 = Floor {
            position: [-1, 1],
            texture: "floor1".to_owned(),
        };

        let floor5 = Floor {
            position: [-2, 0],
            texture: "floor1".to_owned(),
        };

        let floor6 = Floor {
            position: [0, 2],
            texture: "floor1".to_owned(),
        };

        self.world.spawn((ship_entity, floor1));
        self.world.spawn((ship_entity, floor2));
        self.world.spawn((ship_entity, floor3));
        self.world.spawn((ship_entity, floor4));
        self.world.spawn((ship_entity, floor5));
        self.world.spawn((ship_entity, floor6));
    }
}
