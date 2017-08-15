extern crate gus;
extern crate rand;
extern crate chrono;

use gus::*;

use std::io::Write;
use std::fs::File;
use std::thread;
use std::sync::Arc;

pub fn main() {
    let scene = {
        let gray = Beam::red() + Beam::green() + Beam::blue();

        let d_rg = Material::diffuse(Beam::red() + Beam::green());
        let d_gb = Material::diffuse(Beam::green() + Beam::blue());
        let d_br = Material::diffuse(Beam::blue() + Beam::red());
        let e_w = Material::emission(gray);

        let dr = Material::diffuse(gray * 0.01) + Material::reflection(gray * 0.9);

        let r = 100000.0;
        let zp = Sphere::new(V3::new(0.0, 0.0, r + 20.0), r, d_rg);
        let zn = Sphere::new(
            V3::new(0.0, 0.0, -r - 10.0),
            r,
            Material::diffuse(gray * 0.5),
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

    let screen = {
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

        Screen::new(format.clone(), eye)
    };

    let scene_ref = Arc::new(scene);
    let screen_ref = Arc::new(screen);

    let threads: Vec<_> = (0..8).into_iter().map(|_| {
        let scene_ref_clone = scene_ref.clone();
        let screen_ref_clone = screen_ref.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut image = screen_ref_clone.create_image();
            for i in 0..16 {
                screen_ref_clone.sample(&*scene_ref_clone, &mut image, &mut rng);
                println!("sample: {:?}", i + 1);
            }
            image
        })
    }).collect();

    let mut result_image = (*screen_ref).create_image();
    let images: Vec<Image> = threads.into_iter().map(|thread| { thread.join().unwrap() }).collect();
    for &ref image in images.iter() {
        result_image.append(&image);
    }

    let raw = result_image.raw_rgb();

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
