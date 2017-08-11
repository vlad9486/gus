use std::ops::Mul;
use std::ops::Add;
use std::ops::Div;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;

const SIZE: usize = 3;

/// Frequency struct is index in table

#[derive(Copy, Clone)]
pub struct Frequency {
    index: usize
}

impl Frequency {
    pub fn new(index: usize) -> Self {
        Frequency { index: index % Beam::SIZE }
    }
}

/// Density struct is a number of particles in ray

pub enum SingleFate {
    Decay,
    Diffuse,
    Reflect,
    Refract(f32)
}

pub struct Fate {
    pub emission: bool,
    pub single: SingleFate
}

#[derive(Copy, Clone, Default)]
pub struct Density {
    value: f32
}

impl Density {
    pub fn new(value: f32) -> Self {
        Density { value: value }
    }
    
    fn fate(self, mut rng: &mut Rng) -> bool {
        Range::new(0.0f32, 1.0f32).sample(&mut rng) < self.value
    }
    
    fn fate_3way(factor: f32, diffuse: Self, reflect: Self, refract: Self,
            mut rng: &mut Rng) -> SingleFate {
        use self::SingleFate::*;
        
        let fate = Range::new(0.0f32, 1.0f32).sample(&mut rng);
        if fate < diffuse.value {
            Diffuse
        } else if fate < (diffuse + reflect).value {
            Reflect
        } else if fate < (diffuse + reflect + refract).value {
            Refract(factor)
        } else {
            Decay
        }
    }
}

impl Mul for Density {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Density { value: self.value * rhs.value }
    }
}

impl Add for Density {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Density { value: self.value + rhs.value }
    }
}

/// Beam struct is a compound of different Photons

#[derive(Default, Copy, Clone)]
pub struct Beam {
    powers: [Density; SIZE]
}

impl Beam {
    pub const SIZE: usize = SIZE;
    
    pub fn red() -> Self {
        Beam { powers: [Density::new(1.0), Density::new(0.0), Density::new(0.0)] }
    }
    
    pub fn green() -> Self {
        Beam { powers: [Density::new(0.0), Density::new(1.0), Density::new(0.0)] }
    }
    
    pub fn blue() -> Self {
        Beam { powers: [Density::new(0.0), Density::new(0.0), Density::new(1.0)] }
    }
    
    pub fn rgb(self) -> RGB {
        RGB {
            r: (self.clone() * Beam::red()).value,
            g: (self.clone() * Beam::green()).value,
            b: (self.clone() * Beam::blue()).value
        }
    }
    
    pub fn density(&self, frequency: Frequency) -> Density {
        self.powers[frequency.index].clone()
    }
}

impl Mul<Beam> for Beam {
    type Output = Density;
    
    fn mul(self, rhs: Self) -> Self::Output {
        let mut product = Density::default();
        for i in 0..Self::SIZE {
            product = product + self.powers[i] * rhs.powers[i];
        }
        
        product
    }
}

impl Mul<f32> for Beam {
    type Output = Beam;
    
    fn mul(self, rhs: f32) -> Self::Output {
        let mut powers = [Density::default(); Self::SIZE];
        
        for i in 0..Self::SIZE {
            powers[i] = self.powers[i] * Density { value: rhs };
        }
        
        Beam { powers: powers }
    }
}

impl Add<Beam> for Beam {
    type Output = Self;
    
    fn add(self, rhs: Beam) -> Self::Output {
        let mut powers = [Density::default(); Self::SIZE];
        
        for i in 0..Self::SIZE {
            powers[i] = self.powers[i] + rhs.powers[i];
        }
        
        Beam { powers: powers }
    }
}

impl Add<Frequency> for Beam {
    type Output = Self;
    
    fn add(self, rhs: Frequency) -> Self::Output {
        let Beam { powers: mut powers } = self;
        powers[rhs.index] = powers[rhs.index] + Density::new(3.0);
        Beam { powers: powers }
    }
}

/// Material struct

#[derive(Default, Copy, Clone)]
pub struct Material {
    emission: Beam,
    diffuse: Beam,
    reflection: Beam,
    refraction: Beam,
    refraction_factor: Beam
}

impl Material {
    pub fn emission(beam: Beam) -> Self {
        Material {
            emission: beam,
            diffuse: Beam::default(),
            reflection: Beam::default(),
            refraction: Beam::default(),
            refraction_factor: Beam::default()
        }
    }
    
    pub fn diffuse(beam: Beam) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: beam,
            reflection: Beam::default(),
            refraction: Beam::default(),
            refraction_factor: Beam::default()
        }
    }
    
    pub fn reflection(beam: Beam) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: Beam::default(),
            reflection: beam,
            refraction: Beam::default(),
            refraction_factor: Beam::default()
        }
    }
    
    pub fn refraction(beam: Beam, factor: Beam) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: Beam::default(),
            reflection: Beam::default(),
            refraction: beam,
            refraction_factor: factor
        }
    }
    
    pub fn fate(&self, frequency: Frequency, mut rng: &mut Rng) -> Fate {
        Fate {
            emission: self.emission.density(frequency).fate(&mut rng),
            single: Density::fate_3way(
                self.refraction_factor.density(frequency).value,
                self.diffuse.density(frequency),
                self.reflection.density(frequency),
                self.refraction.density(frequency),
                &mut rng
            )
        }
    }
}

impl Add for Material {
    type Output = Self;
    
    fn add(self, rhs: Material) -> Self::Output {
        Material {
            emission: self.emission + rhs.emission,
            diffuse: self.diffuse + rhs.diffuse,
            reflection: self.reflection + rhs.reflection,
            refraction: self.refraction + rhs.refraction,
            refraction_factor: self.refraction_factor + rhs.refraction_factor
        }
    }
}

impl Mul<f32> for Material {
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Material {
            emission: self.emission * rhs,
            diffuse: self.diffuse * rhs,
            reflection: self.reflection * rhs,
            refraction: self.refraction * rhs,
            refraction_factor: self.refraction_factor * rhs
        }
    }
}

/// RGB struct to pass on the screen

#[derive(Default, Copy, Clone)]
pub struct RGB {
    r: f32, g: f32, b: f32
}

impl RGB {
    pub fn r(&self) -> f32 { self.r }
    pub fn g(&self) -> f32 { self.g }
    pub fn b(&self) -> f32 { self.b }
}

impl Add for RGB {
    type Output = Self;
    
    fn add(self, rhs: RGB) -> Self::Output {
        RGB { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl Div<usize> for RGB {
    type Output = Self;
    
    fn div(self, rhs: usize) -> Self::Output {
        let f = rhs as f32;
        RGB { r: self.r / f, g: self.g / f, b: self.b / f }
    }
}
