use crate::vector::Vector;

pub struct Ship {
    pub parts: Vec<ShipParts>,
    pub position: Vector,
    pub rotation: f32,
    pub speed: Vector,
    pub rotation_speed: f32,
}

pub enum ShipParts {
    Floor(Floor),
}

pub struct Floor {
    pub position: [i32; 2],
    pub texture: String,
}
