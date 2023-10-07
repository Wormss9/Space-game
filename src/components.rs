use crate::vector::Vector;

pub struct PlayerControlled {}

#[derive(Debug)]

pub struct Ship {
    pub position: Vector,
    pub rotation: f32,
}

#[derive(Debug)]
pub struct Floor {
    pub position: [i32; 2],
    pub texture: String,
}
