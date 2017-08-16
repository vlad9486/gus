use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;

use std::f64;

pub type M = f64;
pub const M_PI: M = f64::consts::PI;
pub const M_INFINITY: M = f64::INFINITY;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct V3 {
    x: M,
    y: M,
    z: M,
}

impl V3 {
    pub fn new(x: M, y: M, z: M) -> Self {
        V3 { x: x, y: y, z: z }
    }

    pub fn length(self) -> M {
        (self * self).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn cross(self, rhs: Self) -> Self {
        V3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Add for V3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        V3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        V3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<V3> for V3 {
    type Output = M;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<M> for V3 {
    type Output = Self;

    fn mul(self, rhs: M) -> Self::Output {
        V3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<M> for V3 {
    type Output = Self;

    fn div(self, rhs: M) -> Self::Output {
        V3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        V3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
