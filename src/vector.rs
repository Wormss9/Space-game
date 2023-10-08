use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub position: [f32; 2],
}

impl Vector {
    pub fn adjust_ratio(mut self, aspect_ratio: f32) -> Self {
        self.position[0] /= aspect_ratio;
        self
    }
    pub fn adjust_position(self, coordinates: &[i32; 2], scale: f32, rotation: f32) -> Self {
        //Hexagon height is 2
        let x_offset = 8.0 / 9.0 / 1.11; //Hexagon size ratio / texture scale
        let y_offset = 6.0 / 4.0 / 1.11; //Height * vertical spacing / texture scale

        let ur = Self {
            position: [x_offset, y_offset],
        } * coordinates[0] as f32;

        let dr = Self {
            position: [x_offset, -y_offset],
        } * coordinates[1] as f32;

        ((ur + dr) * scale).rotate(rotation) + self
    }
    pub fn rotate(self, rotation: f32) -> Self {
        Self {
            position: [
                self.position[0] * rotation.cos() + self.position[1] * rotation.sin(),
                -self.position[0] * rotation.sin() + self.position[1] * rotation.cos(),
            ],
        }
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

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            position: [self.position[0] * rhs, self.position[1] * rhs],
        }
    }
}
