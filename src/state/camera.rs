use glium::Display;
use hecs::{Entity, World};

use crate::{components::Position, vector::Vector};

#[derive(Debug)]
pub struct Camera {
    pub position: Vector,
    pub scale: f64,
    pub aspect_ratio: f64,
    pub rotation: f64,
    pub focus_object: Option<Entity>,
    pub focus_object_position: Vector,
}

impl Camera {
    pub fn new(display: &Display) -> Self {
        let (x, y) = display.get_framebuffer_dimensions();
        let aspect_ratio = x as f64 / y as f64;

        Self {
            position: Vector::new(0.0, 0.0),
            scale: 0.3,
            aspect_ratio,
            rotation: 0.0,
            focus_object: None,
            focus_object_position: Vector::new(0.0, 0.0),
        }
    }
    pub fn update(&mut self, world: &World) {
        if let Some(focus_object) = self.focus_object {
            self.focus_object_position = world
                .entity(focus_object)
                .unwrap()
                .get::<&Position>()
                .unwrap()
                .position
        }
    }
}
