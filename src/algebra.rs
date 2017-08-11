use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;

#[derive(Copy, Clone)]
pub struct V3 {
    x: f32, y: f32, z: f32
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        V3 { x: x, y: y, z: z }
    }
    
    pub fn length(self) -> f32 {
        (self * self).sqrt()
    }
    
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    
    pub fn cross(self, rhs: Self) -> Self {
        V3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl Add for V3 {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        V3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for V3 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        V3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<V3> for V3 {
    type Output = f32;
    
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f32> for V3 {
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self::Output {
        V3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Div<f32> for V3 {
    type Output = Self;
    
    fn div(self, rhs: f32) -> Self::Output {
        V3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Neg for V3 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        V3 { x: -self.x, y: -self.y, z: -self.z }
    }
}
