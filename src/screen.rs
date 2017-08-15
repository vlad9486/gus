use super::algebra::V3;
use super::algebra::M;

use super::ray::Ray;
use super::ray::PhotonicRay;
use super::scene::Scene;

use super::beam::Frequency;
use super::beam::Beam;
use super::beam::RGB;
use super::beam::Density;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub horizontal_count: usize,
    pub vertical_count: usize,
}

#[derive(Copy, Clone)]
pub struct Eye {
    pub position: V3,
    pub forward: V3,
    pub right: V3,
    pub up: V3,

    pub width: M,
    pub height: M,
    pub distance: M,
}

pub struct Screen {
    format: Size,
    eye: Eye,
}

impl Screen {
    pub fn new(format: Size, eye: Eye) -> Self {
        Screen {
            format: format,
            eye: eye,
        }
    }

    pub fn create_image(&self) -> Image {
        Image::new(self.format)
    }

    pub fn sample(&self, scene: &Scene, image: &mut Image, mut rng: &mut Rng) {
        let format = &self.format;
        let eye = &self.eye;

        for i in 0..format.vertical_count {
            for j in 0..format.horizontal_count {
                let mut beam = Beam::default();

                let direction_calc = |dx: M, dy: M| {
                    let x = eye.width * (((j as M) + dx) / (format.horizontal_count as M) - 0.5);
                    let y = eye.height * (((i as M) + dy) / (format.vertical_count as M) - 0.5);
                    let direction = eye.forward * eye.distance + eye.right * x + eye.up * y;
                    direction.normalize()
                };

                for k in 0..Beam::SIZE {
                    let dx = Range::new(-0.5, 0.5).sample(&mut rng);
                    let dy = Range::new(-0.5, 0.5).sample(&mut rng);

                    let rays = scene.trace(
                        &Ray::new(eye.position, direction_calc(dx, dy), Frequency::new(k)),
                        &mut rng,
                    );
                    beam = rays.into_iter().fold(
                        beam,
                        |beam, ray| beam + ray.frequency(),
                    );
                }
                let rgb = &mut image.data[i * format.horizontal_count + j];
                *rgb = *rgb + beam.rgb()
            }
        }

        image.count = image.count + 1;
    }
}

pub struct Image {
    format: Size,
    data: Vec<RGB>,
    count: usize,
}

impl Image {
    fn new(format: Size) -> Self {
        let capacity = format.horizontal_count * format.vertical_count;
        let mut data = Vec::with_capacity(capacity);

        for _ in 0..(format.vertical_count * format.horizontal_count) {
            data.push(RGB::default())
        }

        Image {
            format: format,
            data: data,
            count: 0,
        }
    }

    pub fn raw_rgb(self, scale: Density) -> Vec<u8> {
        let format = &self.format;
        let capacity = format.horizontal_count * format.vertical_count * 3;

        let mut result = Vec::with_capacity(capacity);
        for &pixel in self.data.iter() {
            (pixel * scale / self.count).update_raw(&mut result);
        }

        result
    }

    pub fn append(&mut self, other: &Self) {
        assert!(self.format == other.format);

        self.count = self.count + other.count;
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] + other.data[i];
        }
    }
}
