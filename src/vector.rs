use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Vector {
    pub position: [f64; 2],
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { position: [x, y] }
    }
    pub fn adjust_ratio(mut self, aspect_ratio: f64) -> Self {
        self.position[0] /= aspect_ratio;
        self
    }
    pub fn adjust_position(self, coordinates: &[i32; 2], scale: f64, rotation: f64) -> Self {
        //Hexagon height is 2
        let x_offset = 8.0 / 9.0 / 1.11; //Hexagon size ratio / texture scale
        let y_offset = 6.0 / 4.0 / 1.11; //Height * vertical spacing / texture scale

        let ur = Self {
            position: [x_offset, y_offset],
        } * coordinates[0] as f64;

        let dr = Self {
            position: [x_offset, -y_offset],
        } * coordinates[1] as f64;

        ((ur + dr) * scale).rotate(rotation) + self.rotate(rotation)
    }
    pub fn rotate(self, rotation: f64) -> Self {
        Self {
            position: [
                self.position[0] * rotation.cos() + self.position[1] * rotation.sin(),
                -self.position[0] * rotation.sin() + self.position[1] * rotation.cos(),
            ],
        }
    }
    pub fn length_sqr(self) -> f64 {
        self.position[0] * self.position[0] + self.position[1] * self.position[1]
    }
    pub fn length(self) -> f64 {
        self.length_sqr().sqrt()
    }
    pub fn as_f32(self) -> [f32; 2] {
        [self.position[0] as f32, self.position[1] as f32]
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: [
                self.position[0] + rhs.position[0],
                self.position[1] + rhs.position[1],
            ],
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.position[0] += rhs.position[0];
        self.position[1] += rhs.position[1];
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            position: [-self.position[0], -self.position[1]],
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            position: [self.position[0] * rhs, self.position[1] * rhs],
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            position: [self.position[0] / rhs, self.position[1] / rhs],
        }
    }
}
