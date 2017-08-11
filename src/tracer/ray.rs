use super::algebra::V3;
use super::beam::Frequency;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;
use std::f32;

#[derive(Clone)]
pub struct Ray {
    position: V3,
    direction: V3,
    frequency: Frequency
}

impl Ray {
    pub fn new(position: V3, direction: V3, frequency: Frequency) -> Self {
        Ray {
            position: position, direction: direction, frequency: frequency
        }
    }
}

pub trait PhotonicRay {
    fn frequency(&self) -> Frequency;
}

pub trait GeometricalRay {
    fn position(&self) -> V3;
    fn direction(&self) -> V3;
    
    fn diffuse(&self, position: V3, normal: V3, rng: &mut Rng) -> Self;
    fn reflect(&self, position: V3, normal: V3) -> Self;
    fn refract(&self, position: V3, normal: V3, factor: f32) -> Self;
}

impl GeometricalRay for Ray {
    fn position(&self) -> V3 { self.position }
    fn direction(&self) -> V3 { self.direction }
    
    fn diffuse(&self, position: V3, normal: V3, mut rng: &mut Rng) -> Self {
        let a = Range::new(0.0f32, f32::consts::PI * 2.0f32).sample(&mut rng);
        let z = Range::new(-1.0f32, 1.0f32).sample(&mut rng);
        let r = (1.0f32 - z * z).sqrt();
        let x = r * a.sin();
        let y = r * a.cos();
        
        let v = V3::new(x, y, z);
        let direction = if v * normal >= 0.0f32 { v } else { v * (-1.0f32) };
        
        Ray { position: position + direction * 0.01, direction: direction, frequency: self.frequency }
    }
    
    fn reflect(&self, position: V3, normal: V3) -> Self {
        let incident = self.direction;
        let dot_product = incident * normal;
        let direction = normal * (-2.0f32 * dot_product) + incident;
        
        Ray { position: position + direction * 0.01, direction: direction, frequency: self.frequency }
    }
    
    fn refract(&self, position: V3, normal: V3, factor: f32) -> Self {
        let incident = self.direction;
        let temp = incident.cross(normal).cross(normal);
        let sinb = temp.length() * factor;
        if sinb < 1.0f32 {
            let cosb = (1.0f32 - sinb * sinb).sqrt();
            let direction = temp * factor - normal * cosb;
            Ray { position: position + direction * 0.01, direction: direction, frequency: self.frequency }
        } else {
            self.reflect(position, normal)
        }
    }
}

impl PhotonicRay for Ray {
    fn frequency(&self) -> Frequency { self.frequency }
}
