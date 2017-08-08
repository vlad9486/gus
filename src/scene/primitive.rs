use super::algebra::V3;
use super::beam::Material;
use super::Ray;

use std::cmp::Ordering;
use std::f32;

pub struct IntersectResult {
    pub position: V3,
    pub normal: V3,
    pub material: Material
}

#[derive(PartialEq)]
pub struct IntersectInfo {
    pub distance: f32,
    r: f32
}

impl Default for IntersectInfo {
    fn default() -> Self {
        IntersectInfo {
            distance: f32::INFINITY,
            r: 0.0f32
        }
    }
}

impl PartialOrd for IntersectInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center: V3,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: V3, radius: f32, material: Material) -> Self {
        Sphere { center: center, radius: radius, material: material }
    }
    
    pub fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let q = self.center - ray.position;
        let p = ray.direction;
        let r = self.radius;
        
        let b = p * q;
        let (r, d) = {
            let s = q * q - r * r;
            (if s >= 0.0f32 { r } else { -r }, b * b - s)
        };
        
        let distance = if d < 0.0f32 {
            None
        } else {
            let t0 = b - d.sqrt();
            let t1 = b + d.sqrt();
                 if t0 >= 0.0f32 { Some(t0) }
            else if t1 >= 0.0f32 { Some(t1) } 
            else { None }
        };
        
        distance.map(|t| { (IntersectInfo { distance: t, r: r }) })
    }
    
    pub fn result(&self, ray: &Ray, info: IntersectInfo) -> IntersectResult {
        let position = ray.position + ray.direction * info.distance;
        let normal = (position - self.center) / info.r;
        IntersectResult {
            position: position,
            normal: normal,
            material: self.material
        }
    }
}
