use super::algebra::V3;
use super::algebra::M;
use super::algebra::M_INFINITY;

use super::beam::Material;

use super::ray::Ray;
use super::ray::GeometricalRay;

use std::cmp::Ordering;

pub struct IntersectResult {
    pub position: V3,
    pub normal: V3,
    pub material: Material,
}

#[derive(PartialEq)]
pub struct IntersectInfo {
    pub distance: M,
    r: M,
}

impl Default for IntersectInfo {
    fn default() -> Self {
        IntersectInfo {
            distance: M_INFINITY,
            r: 0.0,
        }
    }
}

impl PartialOrd for IntersectInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo>;
    fn result(&self, ray: &Ray, info: IntersectInfo) -> IntersectResult;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sphere {
    center: V3,
    radius: M,
    material: Material,
}

impl Sphere {
    pub fn new(center: V3, radius: M, material: Material) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let q = self.center - ray.position();
        let p = ray.direction();
        let r = self.radius;

        let b = p * q;
        let (r, d) = {
            let s = q * q - r * r;
            (if s >= 0.0 { r } else { -r }, b * b - s)
        };

        let distance = if d < 0.0 {
            None
        } else {
            let t0 = b - d.sqrt();
            let t1 = b + d.sqrt();
            if t0 >= 0.0 {
                Some(t0)
            } else if t1 >= 0.0 {
                Some(t1)
            } else {
                None
            }
        };

        distance.map(|t| (IntersectInfo { distance: t, r: r }))
    }

    fn result(&self, ray: &Ray, info: IntersectInfo) -> IntersectResult {
        let position = ray.position() + ray.direction() * info.distance;
        let normal = (position - self.center) / info.r;
        IntersectResult {
            position: position,
            normal: normal,
            material: self.material.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Triangle {
    a: V3,
    b: V3,
    c: V3,
    material: Material,
}

impl Triangle {
    pub fn new(a: V3, b: V3, c: V3, material: Material) -> Self {
        Triangle {
            a: a,
            b: b,
            c: c,
            material: material,
        }
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<IntersectInfo> {
        let pa = self.a - ray.position();
        let pb = self.b - ray.position();
        let pc = self.c - ray.position();

        // adjugated matrix transforms pa, pb, pc into orts scaled by determinant
        let (ia, ib, ic) = V3::adj(pa, pb, pc);

        // coordinates of direction in adjugated space
        let bx = ray.direction() * ia;
        let by = ray.direction() * ib;
        let bz = ray.direction() * ic;

        // orientation of the triangle'a vertices
        let cw = (bx >= 0.0) && (by >= 0.0) && (bz >= 0.0);
        let ccw = (!cw) && (bx <= 0.0) && (by <= 0.0) && (bz <= 0.0);

        if cw || ccw {
            let r = if cw { 1.0 } else { -1.0 };
            // scaled outer normal
            let normal = (pc - pa).cross(pb - pa);
            Some(IntersectInfo {
                // it is not necessary to normalize the normal
                distance: (pa * normal) / (ray.direction() * normal),
                r: r
            })
        } else {
            None
        }
    }

    fn result(&self, ray: &Ray, info: IntersectInfo) -> IntersectResult {
        let position = ray.position() + ray.direction() * info.distance;
        let normal = (self.c - self.a).cross(self.b - self.a).normalize();
        IntersectResult {
            position: position,
            normal: normal,
            material: self.material.clone(),
        }
    }
}
