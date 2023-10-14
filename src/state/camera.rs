use glium::Display;

use crate::vector::Vector;

#[derive(Debug)]
pub struct Camera {
    pub position: Vector,
    pub scale: f64,
    pub aspect_ratio: f64,
    pub rotation: f64,
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
        }
    }
}
