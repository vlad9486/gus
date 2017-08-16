use super::gus::*;
use super::tga::TgaHeader;

use std::path::Path;

use std::io::Write;
use std::io::Read;
use std::fs::File;

use bincode::{serialize, deserialize, Infinite};

pub trait LoadStore: Sized {
    fn load<P>(path: P) -> Option<Self> where P: AsRef<Path>;
    fn store<P>(&self, path: P) where P: AsRef<Path>;
    fn store_tga<P>(self, path: P) where P: AsRef<Path>;
}

impl LoadStore for Image {
    fn load<P>(path: P) -> Option<Self> where P: AsRef<Path> {
        match File::open(path) {
            Ok(mut file) => {
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                Some(deserialize(data.as_slice()).unwrap())
            }
            Err(_) => None
        }
    }

    fn store<P>(&self, path: P) where P: AsRef<Path> {
        let image_encoded: Vec<u8> = serialize(&self, Infinite).unwrap();
        let mut file = File::create(path).unwrap();
        file.write(image_encoded.as_slice()).unwrap();
    }

    fn store_tga<P>(self, path: P) where P: AsRef<Path> {
        let format = self.size();
        let image_header = TgaHeader::rgb(format.horizontal_count, format.vertical_count);
        let mut file = File::create(path).unwrap();
        file.write(serialize(&image_header, Infinite).unwrap().as_slice()).unwrap();
        file.write(self.bitmap(10.0).as_slice()).unwrap();
    }
}
