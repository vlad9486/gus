extern crate gus;
extern crate rand;
extern crate chrono;
extern crate bincode;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod tracer;
mod image;
mod tga;

use gus::*;

use std::io::BufRead;
use std::io::stdin;
use std::fs;

use self::tracer::Tracer;
use self::image::LoadStore;

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

    let path = "out.bgus";
    let path_backup = "out.backup.bgus";
    let tga_path = "out.tga";

    let mut tracer = Tracer::new(scene, screen);
    let image = Image::load(path);
    tracer.start(4, image, |tid, sample| {
        println!("thread: {:?}, sample: {:?}", tid, sample);
    });

    println!("press enter to terminate");

    let mut line = String::new();
    let stdin = stdin();
    let _ = stdin.lock().read_line(&mut line);

    println!("sending stop signal");

    let result_image = tracer.stop();
    let _ = fs::remove_file(path_backup);
    fs::rename(path, path_backup).unwrap();
    result_image.store(path);
    result_image.store_tga(tga_path);
}
