use super::algebra::V3;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;
use std::f32;

pub fn diffuse(normal: V3, mut rng: &mut Rng) -> V3 {
    
    let a = Range::new(0.0f32, f32::consts::PI * 2.0f32).sample(&mut rng);
    let x = a.sin();
    let y = a.cos();
    let z = Range::new(-1.0f32, 1.0f32).sample(&mut rng);
    
    let v = V3::new(x, y, z);
    if v * normal >= 0.0f32 { v } else { v * (-1.0f32) }
}

pub fn reflection(incident: V3, normal: V3) -> V3 {
    let dot_product = incident * normal;
    normal * (-2.0f32 * dot_product) - incident
}

pub fn refraction(incident: V3, normal: V3, factor: f32) -> V3 {
    let temp = incident.cross(normal).cross(normal);
    let sinb = temp.length() * factor;
    if sinb < 1.0f32 {
        let cosb = (1.0f32 - sinb * sinb).sqrt();
        temp * factor - normal * cosb
    } else {
        reflection(incident, normal)
    }
}
