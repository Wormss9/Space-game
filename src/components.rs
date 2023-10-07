pub struct PlayerControlled {}

#[derive(Debug)]

pub struct Ship {
    pub position: [f32; 2],
    pub rotation: f32,
}

#[derive(Debug)]
pub struct Floor {
    pub position: [i32; 2],
    pub texture: String,
}
