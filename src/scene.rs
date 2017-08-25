use super::primitive::Primitive;
use super::primitive::Sphere;
use super::primitive::Triangle;
use super::primitive::IntersectInfo;

use super::ray::Ray;
use super::ray::PhotonicRay;
use super::ray::GeometricalRay;

use std::cmp::Ordering;

use super::beam::SingleFate;

use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    spheres: Vec<Sphere>,
    triangles: Vec<Triangle>,
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>, triangles: Vec<Triangle>) -> Self {
        Scene {
            spheres: spheres,
            triangles: triangles,
        }
    }

    pub fn trace(&self, ray: &Ray, mut rng: &mut Rng) -> Vec<Ray> {
        self.trace_internal(ray, &mut rng, 0)
    }

    fn trace_internal(&self, ray: &Ray, mut rng: &mut Rng, level: usize) -> Vec<Ray> {
        let maximal_level = 7;
        if level == maximal_level {
            return Vec::new();
        }

        fn find_minimal<'a, T>(v: &'a Vec<T>, ray: &Ray) -> Option<(&'a T, IntersectInfo)>
        where
            T: Primitive,
        {
            v.iter()
                .flat_map(|primitive| match primitive.intersect(ray) {
                    Some(info) => Some((primitive, info)),
                    None => None,
                })
                .min_by(|lhs, rhs| {
                    lhs.1.partial_cmp(&rhs.1).unwrap_or(Ordering::Less)
                })
        }

        let sphere = find_minimal(&self.spheres, ray);
        let triangle = find_minimal(&self.triangles, ray);

        let result = match (sphere, triangle) {
            (Some((s, s_info)), Some((t, t_info))) => {
                if s_info < t_info {
                    Some(s.result(ray, s_info))
                } else {
                    Some(t.result(ray, t_info))
                }
            }
            (Some((s, s_info)), None) => Some(s.result(ray, s_info)),
            (None, Some((t, t_info))) => Some(t.result(ray, t_info)),
            _ => None,
        };

        if let Some(result) = result {
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

        } else {
            Vec::new()
        }
    }
}
