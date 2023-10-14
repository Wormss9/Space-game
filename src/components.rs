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
    pub debug: f64,
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

pub enum ShipParts {
    Floor(Floor),
}

pub struct Floor {
    pub position: [i32; 2],
    pub texture: String,
}
