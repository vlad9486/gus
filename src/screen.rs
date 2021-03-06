use super::algebra::V3;
use super::algebra::M;

use super::ray::Ray;
use super::ray::PhotonicRay;
use super::scene::Scene;

use super::beam::Frequency;
use super::beam::Beam;
use super::beam::RGB;
use super::beam::Density;

use std::ops::AddAssign;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub horizontal_count: usize,
    pub vertical_count: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Eye {
    pub position: V3,
    pub forward: V3,
    pub right: V3,
    pub up: V3,

    pub width: M,
    pub height: M,
    pub distance: M,
}

#[derive(Serialize, Deserialize)]
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
        Image::new(self.format.clone())
    }

    pub fn sample(&self, scene: &Scene, image: &mut Image, mut rng: &mut Rng) {
        let format = &self.format;
        let eye = &self.eye;

        let red = Beam::red();
        let green = Beam::green();
        let blue = Beam::blue();

        for i in 0..format.vertical_count {
            for j in 0..format.horizontal_count {

                let direction_calc = |dx: M, dy: M| {
                    let x = eye.width * (((j as M) + dx) / (format.horizontal_count as M) - 0.5);
                    let y = eye.height * (((i as M) + dy) / (format.vertical_count as M) - 0.5);
                    let direction = eye.forward * eye.distance + eye.right * x + eye.up * y;
                    direction.normalize()
                };

                let beam = (0..Beam::SIZE).into_iter().fold(
                    Beam::default(),
                    |beam, k| {
                        let dx = Range::new(-0.5, 0.5).sample(&mut rng);
                        let dy = Range::new(-0.5, 0.5).sample(&mut rng);

                        scene
                            .trace(
                                &Ray::new(eye.position, direction_calc(dx, dy), Frequency::new(k)),
                                &mut rng,
                            )
                            .into_iter()
                            .fold(beam, |beam, ray| beam + ray.frequency())
                    },
                );

                let rgb = &mut image.data[i * format.horizontal_count + j];
                *rgb = rgb.clone() +
                    RGB::new(
                        beam.clone() * &red,
                        beam.clone() * &green,
                        beam.clone() * &blue,
                    )
            }
        }

        image.count = image.count + 1;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    format: Size,
    data: Vec<RGB>,
    count: usize,
}

impl Image {
    fn new(format: Size) -> Self {
        let data = (0..(format.vertical_count * format.horizontal_count))
            .into_iter()
            .map(|_| RGB::default())
            .collect();

        Image {
            format: format,
            data: data,
            count: 0,
        }
    }

    pub fn bitmap(self, scale: Density) -> Vec<u8> {
        let format = &self.format;
        let capacity = format.horizontal_count * format.vertical_count * 3;

        let mut result = Vec::with_capacity(capacity);
        for &ref pixel in self.data.iter() {
            (pixel.clone() * scale / self.count).update_raw(&mut result);
        }

        result
    }

    pub fn size(&self) -> Size {
        self.format.clone()
    }
}

impl AddAssign<Image> for Image {
    fn add_assign(&mut self, rhs: Image) {
        assert!(self.format == rhs.format);

        self.count += rhs.count;
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() + rhs.data[i].clone();
        }
    }
}
