use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub position: [f32; 2],
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

impl Mul<[[f32; 2]; 2]> for Vector {
    type Output = Self;

    fn mul(self, rhs: [[f32; 2]; 2]) -> Self::Output {
        Self {
            position: [
                self.position[0] * rhs[0][0] + self.position[1] * rhs[0][1],
                self.position[0] * rhs[1][0] + self.position[1] * rhs[1][1],
            ],
        }
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
