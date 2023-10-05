use std::collections::HashMap;

use glium::Display;
use hecs::World;

use self::{clock::Clock, game_state::GameState, textures::Textures};

mod clock;
mod game_state;
mod textures;

pub struct State {
    // pub calendar: GameCalendar,
    pub clock: Clock,
    pub world: World,
    pub textures: Textures,
    // pub sounds: HashMap<String, glium::AudioBuffer>,
    pub game_state: GameState,
    pub programs: HashMap<String, glium::Program>,
}

impl State {
    pub fn new(display: &Display) -> Self {
        Self {
            clock: Clock::new(),
            world: World::new(),
            textures: Textures::new(display),
            game_state: GameState::MainMenu,
            programs: HashMap::new(),
        }
    }
}
