use super::algebra::V3;
use super::scene::Ray;
use super::scene::Scene;

use super::beam::Frequency;
use super::beam::Beam;
use super::beam::RGB;

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Format {
    pub horizontal_count: usize,
    pub vertical_count: usize
}

#[derive(Copy, Clone)]
pub struct Eye {
    pub position: V3,
    pub forward: V3,
    pub right: V3,
    pub up: V3,
    
    pub width: f32,
    pub height: f32,
    pub distance: f32
}

pub struct Screen {
    format: Format,
    initial: Vec<Ray>
}

impl Screen {
    pub fn new(format: Format, eye: Eye) -> Self {
        let capacity = format.horizontal_count * format.vertical_count;
        let mut rays = Vec::with_capacity(capacity);
        
        for i in 0..format.vertical_count {
            for j in 0..format.horizontal_count {
                let x = eye.width * ((j as f32) / (format.horizontal_count as f32) - 0.5f32);
                let y = eye.height * ((i as f32) / (format.vertical_count as f32) - 0.5f32);
                let direction = eye.forward * eye.distance + eye.right * x + eye.up * y;
                let direction = direction.normalize();
                
                for k in 0..Beam::SIZE {
                    rays.push(Ray {
                        position: eye.position,
                        direction: direction,
                        frequency: Frequency::new(k)
                    });
                }
            }
        }
        
        Screen {
            format: format,
            initial: rays
        }
    }
    
    pub fn sample(&self, scene: &Scene, sample: &mut Sample, mut rng: &mut Rng) {
        let format = &self.format;
        
        for i in 0..format.vertical_count {
            for j in 0..format.horizontal_count {
                let mut beam = Beam::default();
                for k in 0..Beam::SIZE {
                    let index = (i * format.horizontal_count + j) * Beam::SIZE + k;
                    let ray = &self.initial[index];
                    let rays: Vec<Ray> = scene.trace(*ray, &mut rng);
                    beam = rays.into_iter().fold(beam, |beam, ray| { beam + ray.frequency });
                }
                let mut rgb = &mut sample.data[i * format.horizontal_count + j];
                *rgb = *rgb + beam.rgb()
            }
        }
        
        sample.count = sample.count + 1;
    }
}

pub struct Sample {
    format: Format,
    data: Vec<RGB>,
    count: usize
}

impl Sample {
    pub fn new(format: Format) -> Self {
        let capacity = format.horizontal_count * format.vertical_count;
        let mut data = Vec::with_capacity(capacity);
        
        for _ in 0..(format.vertical_count * format.horizontal_count) {
            data.push(RGB::default())
        }
        
        Sample {
            format: format,
            data: data,
            count: 0
        }
    }
    
    pub fn raw_rgb(self) -> Vec<u8> {
        let format = &self.format;
        let capacity = format.horizontal_count * format.vertical_count * 3;
        
        let mut result = Vec::with_capacity(capacity);
        for &pixel in self.data.iter() {
            let rgb = pixel / self.count;
            
            let to_byte = |a: f32| -> u8 { 
                if a > 1.0 { 255 } else if a < 0.0 { 0 } else { (a * 255.0) as u8 }
            };
            
            result.push(to_byte(rgb.b()));
            result.push(to_byte(rgb.g()));
            result.push(to_byte(rgb.r()));
        }
        
        result
    }
}
