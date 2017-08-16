use super::primitive::Sphere;

use super::ray::Ray;
use super::ray::PhotonicRay;
use super::ray::GeometricalRay;

use super::beam::SingleFate;

use std::cmp::Ordering;

use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>) -> Self {
        Scene { spheres: spheres }
    }

    pub fn trace(&self, ray: &Ray, mut rng: &mut Rng) -> Vec<Ray> {
        self.trace_internal(ray, &mut rng, 0)
    }

    fn trace_internal(&self, ray: &Ray, mut rng: &mut Rng, level: usize) -> Vec<Ray> {
        let maximal_level = 7;
        if level == maximal_level {
            return Vec::new();
        }

        let minimal = self.spheres
            .iter()
            .map(|sphere| (sphere, sphere.intersect(ray)))
            .min_by(|lhs, rhs| match (&lhs.1, &rhs.1) {
                (&None, &None) => Ordering::Equal,
                (&Some(_), &None) => Ordering::Less,
                (&None, &Some(_)) => Ordering::Greater,
                (&Some(ref l), &Some(ref r)) => l.partial_cmp(&r).unwrap(),
            });

        match minimal {
            Some((sphere, Some(m))) => {
                let result = sphere.result(ray, m);
                let mut rays = Vec::with_capacity(8);

                let fate = result.material.fate(&ray.frequency(), &mut rng);

                if fate.emission {
                    rays.push((*ray).clone());
                }

                use self::SingleFate::*;
                let ray = match fate.single {
                    Decay => None,
                    Diffuse => Some(ray.diffuse(result.position, result.normal, &mut rng)),
                    Reflect => Some(ray.reflect(result.position, result.normal)),
                    Refract(factor) => Some(ray.refract(result.position, result.normal, factor)),
                };

                if let Some(ray) = ray {
                    rays.append(&mut self.trace_internal(&ray, rng, level + 1));
                }

                rays
            }
            _ => Vec::new(),
        }
    }
}
