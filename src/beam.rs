use std::ops::Mul;
use std::ops::Add;
use std::ops::Div;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::Range;

use super::color;

const SIZE: usize = 24;

/// Frequency struct is index in table
#[derive(Copy, Clone)]
pub struct Frequency {
    index: usize,
}

impl Frequency {
    pub fn new(index: usize) -> Self {
        Frequency { index: index % Beam::SIZE }
    }
}

/// Density is a number of particles in ray

pub type Density = f32;
pub type Factor = f64;

/// Fate

pub enum SingleFate {
    Decay,
    Diffuse,
    Reflect,
    Refract(Factor),
}

pub struct Fate {
    pub emission: bool,
    pub single: SingleFate,
}

fn fate(value: Density, mut rng: &mut Rng) -> bool {
    Range::new(0.0 as Density, 1.0).sample(&mut rng) < value
}

impl SingleFate {

    fn new(
        factor: Factor,
        diffuse: Density,
        reflect: Density,
        refract: Density,
        mut rng: &mut Rng,
    ) -> Self {
        use self::SingleFate::*;

        let fate = Range::new(0.0 as Density, 1.0).sample(&mut rng);
        if fate < diffuse {
            Diffuse
        } else if fate < diffuse + reflect {
            Reflect
        } else if fate < diffuse + reflect + refract {
            Refract(factor)
        } else {
            Decay
        }
    }
}

/// Beam struct is a compound of different Photons
#[derive(Default, Copy, Clone)]
pub struct Beam {
    powers: [Density; SIZE],
}

impl Beam {
    pub const SIZE: usize = SIZE;

    fn populate(index: usize) -> Self {
        let mut powers = [0.0; Self::SIZE];
        for i in 0..Self::SIZE {
            let value = {
                let j = (i * color::TABLE_SIZE) / Self::SIZE;
                let table = color::table();
                let rgb = table[j].1;
                match index {
                    0 => rgb.r,
                    1 => rgb.g,
                    _ => rgb.b
                }
            };

            powers[i] = value;
        }

        let beam = Beam { powers: powers };
        let length = (beam * beam).sqrt();
        beam * (1.0 / length)
    }

    pub fn red() -> Self {
        Self::populate(0)
    }

    pub fn green() -> Self {
        Self::populate(1)
    }

    pub fn blue() -> Self {
        Self::populate(2)
    }

    pub fn rgb(self) -> RGB {
        RGB {
            r: self.clone() * Beam::red(),
            g: self.clone() * Beam::green(),
            b: self.clone() * Beam::blue(),
        }
    }

    fn density(&self, frequency: Frequency) -> Density {
        self.powers[frequency.index]
    }
}

impl Mul<Beam> for Beam {
    type Output = Density;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut product = 0.0;
        for i in 0..Self::SIZE {
            product = product + self.powers[i] * rhs.powers[i];
        }

        product
    }
}

impl Mul<Density> for Beam {
    type Output = Beam;

    fn mul(self, rhs: Density) -> Self::Output {
        let mut powers = [0.0; Self::SIZE];

        for i in 0..Self::SIZE {
            powers[i] = self.powers[i] * rhs;
        }

        Beam { powers: powers }
    }
}

impl Add<Beam> for Beam {
    type Output = Self;

    fn add(self, rhs: Beam) -> Self::Output {
        let mut powers = [0.0; Self::SIZE];

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
        powers[rhs.index] = powers[rhs.index] + 1.0;
        Beam { powers: powers }
    }
}

#[derive(Copy, Clone)]
pub struct BeamRefract {
    powers: [Factor; SIZE],
}

impl BeamRefract {
    pub const SIZE: usize = SIZE;

    pub fn identity() -> Self {
        BeamRefract {
            powers: [1.0; SIZE]
        }
    }

    fn factor(&self, frequency: Frequency) -> Factor {
        self.powers[frequency.index]
    }
}

impl Default for BeamRefract {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul<Factor> for BeamRefract {
    type Output = BeamRefract;

    fn mul(self, rhs: Factor) -> Self::Output {
        let mut powers = [0.0; Self::SIZE];

        for i in 0..Self::SIZE {
            powers[i] = self.powers[i] * rhs;
        }

        BeamRefract { powers: powers }
    }
}

impl Add<BeamRefract> for BeamRefract {
    type Output = Self;

    fn add(self, rhs: BeamRefract) -> Self::Output {
        let mut powers = [0.0; Self::SIZE];

        for i in 0..Self::SIZE {
            powers[i] = self.powers[i] + rhs.powers[i];
        }

        BeamRefract { powers: powers }
    }
}

/// Material struct
#[derive(Default, Copy, Clone)]
pub struct Material {
    emission: Beam,
    diffuse: Beam,
    reflection: Beam,
    refraction: Beam,
    refraction_factor: BeamRefract,
}

impl Material {
    pub fn emission(beam: Beam) -> Self {
        Material {
            emission: beam,
            diffuse: Beam::default(),
            reflection: Beam::default(),
            refraction: Beam::default(),
            refraction_factor: BeamRefract::default(),
        }
    }

    pub fn diffuse(beam: Beam) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: beam,
            reflection: Beam::default(),
            refraction: Beam::default(),
            refraction_factor: BeamRefract::default(),
        }
    }

    pub fn reflection(beam: Beam) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: Beam::default(),
            reflection: beam,
            refraction: Beam::default(),
            refraction_factor: BeamRefract::default(),
        }
    }

    pub fn refraction(beam: Beam, factor: BeamRefract) -> Self {
        Material {
            emission: Beam::default(),
            diffuse: Beam::default(),
            reflection: Beam::default(),
            refraction: beam,
            refraction_factor: factor,
        }
    }

    pub fn fate(&self, frequency: Frequency, mut rng: &mut Rng) -> Fate {
        Fate {
            emission: fate(self.emission.density(frequency), &mut rng),
            single: SingleFate::new(
                self.refraction_factor.factor(frequency),
                self.diffuse.density(frequency),
                self.reflection.density(frequency),
                self.refraction.density(frequency),
                &mut rng,
            ),
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
            refraction_factor: self.refraction_factor + rhs.refraction_factor,
        }
    }
}

/// RGB struct to pass on the screen
#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct RGB {
    r: Density,
    g: Density,
    b: Density,
}

impl RGB {
    pub fn new(r: Density, g: Density, b: Density) -> Self {
        RGB { r: r, g: g, b: b }
    }

    pub fn update_raw(self, raw: &mut Vec<u8>) {
        let to_byte = |a: Density| -> u8 {
            if a > 1.0 {
                255
            } else if a < 0.0 {
                0
            } else {
                (a * 255.0) as u8
            }
        };

        raw.push(to_byte(self.b));
        raw.push(to_byte(self.g));
        raw.push(to_byte(self.r));
    }
}

impl Add for RGB {
    type Output = Self;

    fn add(self, rhs: RGB) -> Self::Output {
        RGB {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<Density> for RGB {
    type Output = Self;

    fn mul(self, rhs: Density) -> Self::Output {
        RGB {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<usize> for RGB {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        let f = rhs as Density;
        RGB {
            r: self.r / f,
            g: self.g / f,
            b: self.b / f,
        }
    }
}
