use super::algebra::V3;
use super::algebra::M;
use super::algebra::M_PI;

use super::beam::Frequency;
use super::beam::Factor;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;

#[derive(Clone)]
pub struct Ray {
    position: V3,
    direction: V3,
    frequency: Frequency,
}

impl Ray {
    pub const EPS: M = 0.01;

    pub fn new(position: V3, direction: V3, frequency: Frequency) -> Self {
        Ray {
            position: position,
            direction: direction,
            frequency: frequency,
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
    fn refract(&self, position: V3, normal: V3, factor: Factor) -> Self;
}

impl GeometricalRay for Ray {
    fn position(&self) -> V3 {
        self.position
    }
    fn direction(&self) -> V3 {
        self.direction
    }

    fn diffuse(&self, position: V3, normal: V3, mut rng: &mut Rng) -> Self {
        let a = Range::new(0.0, M_PI * 2.0).sample(&mut rng);
        let z = Range::new(-1.0, 1.0).sample(&mut rng);
        let r = ((1.0 - z * z) as M).sqrt();
        let x = r * a.sin();
        let y = r * a.cos();

        let v = V3::new(x, y, z);
        let direction = if v * normal >= 0.0 {
            v
        } else {
            v * (-1.0)
        };

        Ray {
            position: position + direction * Self::EPS,
            direction: direction,
            frequency: self.frequency.clone(),
        }
    }

    fn reflect(&self, position: V3, normal: V3) -> Self {
        let incident = self.direction;
        let dot_product = incident * normal;
        let direction = normal * (-2.0 * dot_product) + incident;

        Ray {
            position: position + direction * Self::EPS,
            direction: direction,
            frequency: self.frequency.clone(),
        }
    }

    fn refract(&self, position: V3, normal: V3, factor: Factor) -> Self {
        let incident = self.direction;
        let temp = incident.cross(normal).cross(normal);
        let sinb = temp.length() * factor;
        if sinb < 1.0 {
            let cosb = (1.0 - sinb * sinb).sqrt();
            let direction = temp * factor - normal * cosb;
            Ray {
                position: position + direction * Self::EPS,
                direction: direction,
                frequency: self.frequency.clone(),
            }
        } else {
            self.reflect(position, normal)
        }
    }
}

impl PhotonicRay for Ray {
    fn frequency(&self) -> Frequency {
        self.frequency.clone()
    }
}
