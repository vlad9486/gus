extern crate gus;
extern crate rand;
extern crate chrono;
extern crate bincode;

use gus::*;

use bincode::{serialize, deserialize, Infinite};

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
        let e_w = Material::emission(gray.clone());

        let dr = Material::diffuse(gray.clone() * 0.01) + Material::reflection(gray.clone() * 0.9);

        let r = 100000.0;
        let zp = Sphere::new(V3::new(0.0, 0.0, r + 20.0), r, d_rg.clone());
        let zn = Sphere::new(
            V3::new(0.0, 0.0, -r - 10.0),
            r,
            Material::diffuse(gray.clone() * 0.5),
        );
        let yp = Sphere::new(V3::new(0.0, r + 10.0, 0.0), r, d_gb.clone());
        let yn = Sphere::new(V3::new(0.0, -r - 10.0, 0.0), r, d_gb.clone());
        let xp = Sphere::new(V3::new(r + 10.0, 0.0, 0.0), r, d_br.clone());
        let xn = Sphere::new(V3::new(-r - 10.0, 0.0, 0.0), r, d_br.clone());

        let source = Sphere::new(V3::new(0.0, 1000.0 + 9.98, 0.0), 1000.0, e_w.clone());

        let ml = Sphere::new(V3::new(-2.0, 0.0, 15.0), 2.0, dr.clone());
        let mr = Sphere::new(V3::new(3.5, -1.0, 12.0), 3.0, dr.clone());
        let mo = Sphere::new(V3::new(-1.5, 3.0, 9.0), 3.5, dr.clone());

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

    let threads: Vec<_> = (0..4).into_iter().map(|_| {
        let scene_ref_clone = scene_ref.clone();
        let screen_ref_clone = screen_ref.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut image = screen_ref_clone.create_image();
            for i in 0..2 {
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

    {
        let image_encoded: Vec<u8> = serialize(&result_image, Infinite).unwrap();
        let mut file = File::create("out.gus").unwrap();
        let _ = file.write(image_encoded.as_slice()).unwrap();
    }

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
    let _ = file.write(result_image.bitmap(10.0).as_slice()).unwrap();
}
