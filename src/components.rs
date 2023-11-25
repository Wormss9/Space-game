use crate::vector::Vector;

#[derive(Debug, Clone)]
pub struct GravityMass {
    pub mass: f64,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub position: Vector,
    pub acceleration: Vector,
    pub speed: Vector,
}

pub struct Rotation {
    pub rotation: f64,
    pub rotation_speed: f64,
}

pub struct Planet {
    pub radius: f64,
}

pub struct Ship {
    pub parts: Vec<ShipParts>,
}

impl Ship {
    pub fn sort_parts(&mut self) {
        self.parts.sort_by_key(|part| part.layer());
    }
}

pub enum ShipParts {
    Floor(Tile),
    Wall(Tile),
}

impl ShipParts {
    pub fn layer(&self) -> usize {
        match self {
            ShipParts::Floor(_) => 0,
            ShipParts::Wall(_) => 1,
        }
    }
}

pub struct Tile {
    pub position: [i32; 2],
    pub texture: String,
}
