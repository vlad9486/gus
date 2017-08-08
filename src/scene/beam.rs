use std::ops::Mul;
use std::ops::Add;
use std::ops::Div;

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

#[derive(Copy, Clone, Default)]
pub struct Density {
    pub value: f32
}

impl Density {
    pub fn new(value: f32) -> Self {
        Density { value: value }
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
    reflection: Beam/*,
    refraction: Beam,
    refraction_factor: Beam*/
}

impl Material {
    pub fn emission(beam: Beam) -> Self {
        Material {
            emission: beam,
            diffuse: Beam::default(),
            reflection: Beam::default()
        }
    }
    
    pub fn diffuse(beam: Beam) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: beam,
            reflection: Beam::default()
        }
    }
    
    pub fn reflection(beam: Beam) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: Beam::default(),
            reflection: beam
        }
    }
    
    pub fn density(&self, frequency: Frequency) -> (Density, Density, Density) {
        (
            self.emission.density(frequency),
            self.diffuse.density(frequency),
            self.reflection.density(frequency)
        )
    }
}

impl Add for Material {
    type Output = Self;
    
    fn add(self, rhs: Material) -> Self::Output {
        Material {
            emission: self.emission + rhs.emission,
            diffuse: self.diffuse + rhs.diffuse,
            reflection: self.reflection + rhs.reflection
        }
    }
}

impl Mul<f32> for Material {
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Material {
            emission: self.emission * rhs,
            diffuse: self.diffuse * rhs,
            reflection: self.reflection * rhs
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
