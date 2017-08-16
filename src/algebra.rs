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

    pub fn adj(a: Self, b: Self, c: Self) -> (Self, Self, Self) {
        let ia = V3::new(b.y * c.z - b.z * c.y, c.y * a.z - c.z * a.y, a.y * b.z - a.z * b.y);
        let ib = V3::new(b.z * c.x - b.x * c.z, c.z * a.x - c.x * a.z, a.z * b.x - a.x * b.z);
        let ic = V3::new(b.x * c.y - b.y * c.x, c.x * a.y - c.y * a.x, a.x * b.y - a.y * b.x);

        (ia, ib, ic)
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
