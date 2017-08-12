extern crate gus;
extern crate rand;
extern crate chrono;

use gus::*;

use std::io::Write;
use std::fs::File;

pub fn main() {
    let scene = {
        let gray = Beam::red() + Beam::green() + Beam::blue();

        let d_rg = Material::diffuse(Beam::red() + Beam::green());
        let d_gb = Material::diffuse(Beam::green() + Beam::blue());
        let d_br = Material::diffuse(Beam::blue() + Beam::red());
        let e_w = Material::emission(gray);

        let dr = Material::diffuse(gray) * 0.01 + Material::reflection(gray) * 0.9;

        let r = 100000.0;
        let zp = Sphere::new(V3::new(0.0, 0.0, r + 20.0), r, d_rg);
        let zn = Sphere::new(
            V3::new(0.0, 0.0, -r - 10.0),
            r,
            Material::diffuse(gray) * 0.5,
        );
        let yp = Sphere::new(V3::new(0.0, r + 10.0, 0.0), r, d_gb);
        let yn = Sphere::new(V3::new(0.0, -r - 10.0, 0.0), r, d_gb);
        let xp = Sphere::new(V3::new(r + 10.0, 0.0, 0.0), r, d_br);
        let xn = Sphere::new(V3::new(-r - 10.0, 0.0, 0.0), r, d_br);

        let source = Sphere::new(V3::new(0.0, 1000.0 + 9.98, 0.0), 1000.0, e_w);

        let ml = Sphere::new(V3::new(-2.0, 0.0, 15.0), 2.0, dr);
        let mr = Sphere::new(V3::new(3.5, -1.0, 12.0), 3.0, dr);
        let mo = Sphere::new(V3::new(-1.5, 3.0, 9.0), 3.5, dr);

        Scene::new(vec![zp, zn, yp, yn, xp, xn, ml, mr, mo, source])
    };

    let format = Size {
        horizontal_count: 1920,
        vertical_count: 1080,
    };

    let eye = Eye {
        position: V3::new(0.0, 0.0, -9.0),
        forward: V3::new(0.0, 0.0, 1.0),
        right: V3::new(1.0, 0.0, 0.0),
        up: V3::new(0.0, 1.0, 0.0),

        width: 1.6,
        height: 0.9,
        distance: 1.5,
    };

    let screen = Screen::new(format.clone(), eye);

    let mut rng = rand::thread_rng();

    let raw = {
        let mut sample = Image::new(format.clone());
        for i in 0..4 {
            screen.sample(&scene, &mut sample, &mut rng);
            println!("sample: {:?}", i + 1);
        }
        sample.raw_rgb()
    };

    // TODO: use some third party for image format
    let header = vec![
        0u8,
        0u8,
        2u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        //width   height
        0u8,
        0u8,
        0u8,
        0u8,
        128u8,
        7u8,
        56u8,
        4u8,
        24u8,
        0u8,
    ];
    let mut file = File::create("out.tga").unwrap();
    let _ = file.write(header.as_slice()).unwrap();
    let _ = file.write(raw.as_slice()).unwrap();
}
