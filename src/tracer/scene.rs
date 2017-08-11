
use super::algebra::V3;
use super::geometry::diffuse;
use super::geometry::reflection;
use super::geometry::refraction;

use super::beam::Frequency;

use super::primitive::Sphere;

use std::cmp::Ordering;

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Ray {
    pub position: V3,
    pub direction: V3,
    pub frequency: Frequency
}

pub struct Scene {
    spheres: Vec<Sphere>
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>) -> Self {
        Scene { spheres: spheres }
    }
    
    pub fn trace(&self, ray: Ray, mut rng: &mut Rng) -> Vec<Ray> {
        self.trace_internal(ray, &mut rng, 0)
    }
    
    fn trace_internal(&self, ray: Ray, mut rng: &mut Rng, level: usize) -> Vec<Ray> {
        let maximal_level = 8;
        if level == maximal_level {
            return Vec::new();
        }
        
        let minimal = self.spheres.iter().map(|sphere| {
            (sphere, sphere.intersect(&ray))
        }).min_by(|lhs, rhs| {
            match (&lhs.1, &rhs.1) {
                (&None, &None) => Ordering::Equal,
                (&Some(_), &None) => Ordering::Less,
                (&None, &Some(_)) => Ordering::Greater,
                (&Some(ref l), &Some(ref r)) => l.partial_cmp(&r).unwrap()
            }
        });
        
        match minimal {
            Some((sphere, Some(m))) => {
                let result = sphere.result(&ray, m);
                let mut rays = Vec::with_capacity(8);
                
                let (e_fate, d_fate, r_fate, _) = result.material.density(ray.frequency);
                
                if e_fate.fate(&mut rng) {
                    rays.push(ray);
                }
                
                if d_fate.fate(&mut rng) {
                    let direction = diffuse(result.normal, &mut rng);
                    let ray = Ray {
                        position: result.position + direction * 0.01,
                        direction: direction,
                        frequency: ray.frequency
                    };
                    rays.append(&mut self.trace_internal(ray, rng, level + 1));
                }
                
                if r_fate.fate(&mut rng) {
                    let direction = reflection(ray.direction, result.normal);
                    let ray = Ray {
                        position: result.position + direction * 0.01,
                        direction: direction,
                        frequency: ray.frequency
                    };
                    rays.append(&mut self.trace_internal(ray, rng, level + 1));
                }
                
                rays
            }
            _ => Vec::new()
        }
    }
}
