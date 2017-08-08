pub mod algebra;
mod geometry;
pub mod beam;
pub mod primitive;
pub mod screen;

use self::algebra::V3;
use self::geometry::diffuse;
use self::geometry::reflection;
use self::geometry::refraction;

use self::beam::Frequency;

use self::primitive::Sphere;

use std::cmp::Ordering;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;

#[derive(Copy, Clone)]
pub struct Ray {
    position: V3,
    direction: V3,
    frequency: Frequency
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
                
                let (e_fate, d_fate, r_fate) = result.material.density(ray.frequency);
                
                let fate = Range::new(0.0f32, 1.0f32).sample(&mut rng);
                if fate < e_fate.value {
                    rays.push(ray);
                }
                
                let fate = Range::new(0.0f32, 1.0f32).sample(&mut rng);
                if fate < d_fate.value {
                    let direction = diffuse(result.normal, &mut rng);
                    let ray = Ray {
                        position: result.position + direction * 0.01,
                        direction: direction,
                        frequency: ray.frequency
                    };
                    rays.append(&mut self.trace_internal(ray, rng, level + 1));
                }
                
                let fate = Range::new(0.0f32, 1.0f32).sample(&mut rng);
                if fate < r_fate.value {
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
