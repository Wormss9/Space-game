use crate::vector::Vector;

pub enum CelestialBodies {
    Star(Star),
    Planet(Planet),
}

pub struct Star {
    mass: f32,
    position: Vector,
    radius: f32,
    brightness: f32,
}

pub struct Planet {
    mass: f32,
    position: Vector,
    radius: f32,
}

pub trait CelestialBody {
    fn get_mass() -> f32;
    fn get_position() -> Vector;
}

pub trait Renderable{
    fn render();
}