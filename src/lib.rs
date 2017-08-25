#![forbid(unsafe_code)]
#![allow(non_shorthand_field_patterns)]

#[macro_use]
extern crate serde_derive;

extern crate rand;

mod algebra;
mod beam;
mod primitive;
mod screen;
mod scene;
mod ray;
mod color;

pub use self::algebra::V3;
pub use self::beam::Beam;
pub use self::beam::BeamRefract;
pub use self::beam::Material;
pub use self::scene::Scene;
pub use self::primitive::Sphere;
pub use self::primitive::Triangle;
pub use self::screen::Screen;
pub use self::screen::Image;
pub use self::screen::Eye;
pub use self::screen::Size;
