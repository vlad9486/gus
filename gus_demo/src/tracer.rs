use super::gus::*;
use super::rand;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

pub struct Tracer {
    scene: Arc<Scene>,
    screen: Arc<Screen>,
    threads: Vec<(thread::JoinHandle<Image>, mpsc::Sender<()>)>,
    image: Option<Image>,
}

pub struct Status {
    pub count: usize,
    pub seconds_per_sample: f32,
}

impl Tracer {
    pub fn new(scene: Scene, screen: Screen) -> Self {
        /*let scene = {
            let mut file = File::open(scene_filename)?;
            let mut string = String::new();
            file.read_to_string(&mut string)?;
            deserialize(string.as_bytes()).unwrap()
        };*/

        Tracer {
            scene: Arc::new(scene),
            screen: Arc::new(screen),
            threads: Vec::new(),
            image: None,
        }
    }

    pub fn start(&mut self, number_of_threads: usize, image: Option<Image>) {
        self.image = image;
        self.threads = (0..number_of_threads).into_iter().map(|i| {
            let (tx, rx) = mpsc::channel();
            let scene_ref_clone = self.scene.clone();
            let screen_ref_clone = self.screen.clone();
            (thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let mut image = screen_ref_clone.create_image();
                let mut j = 0usize;
                loop {
                    screen_ref_clone.sample(&*scene_ref_clone, &mut image, &mut rng);
                    j = j + 1;
                    println!("thread: {:?}, sample: {:?}", i, j);

                    match rx.try_recv() {
                        Ok(_) | Err(mpsc::TryRecvError::Disconnected) => break (),
                        Err(mpsc::TryRecvError::Empty) => ()
                    }
                }
                image
            }), tx)
        }).collect();
    }

    pub fn stop(self) -> Image {
        for &(_, ref tx) in self.threads.iter() {
            tx.send(()).unwrap();
        }

        let images: Vec<Image> = self.threads.into_iter().map(|pair| {
            let (thread, _) = pair;
            thread.join().unwrap()
        }).collect();

        let mut result_image = self.image.unwrap_or(self.screen.create_image());
        for image in images.into_iter() {
            result_image.append(image);
        }

        result_image
    }

    pub fn status(&self) -> Status {
        unimplemented!()
    }
}
