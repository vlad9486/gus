use super::gus::*;
use super::rand;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

use chrono::Utc;
use chrono::Timelike;

pub struct Tracer {
    scene: Arc<Scene>,
    screen: Arc<Screen>,
    threads: Vec<(thread::JoinHandle<Image>, mpsc::Sender<()>)>,
    image: Option<Image>,
}

impl Tracer {
    pub fn new(scene: Scene, screen: Screen) -> Self {
        Tracer {
            scene: Arc::new(scene),
            screen: Arc::new(screen),
            threads: Vec::new(),
            image: None,
        }
    }

    pub fn start<Report>(&mut self, number_of_threads: usize, image: Option<Image>, report: Report)
            where Report: Fn(usize, usize, f64) + Send + Sync + 'static {
        let report = Arc::new(report);
        self.image = image;
        self.threads = (0..number_of_threads).into_iter().map(|i| {
            let (tx, rx) = mpsc::channel();
            let scene_ref_clone = self.scene.clone();
            let screen_ref_clone = self.screen.clone();
            let report_ref_clone = report.clone();
            (thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let mut image = screen_ref_clone.create_image();
                let time = || {
                    let t = Utc::now();
                    (t.timestamp() as f64) + (t.nanosecond() as f64) / 1_000_000_000.0
                };
                let mut j = 0usize;
                let mut last: f64 = time();
                loop {
                    screen_ref_clone.sample(&*scene_ref_clone, &mut image, &mut rng);
                    let temp = time();
                    report_ref_clone(i, j, temp - last);
                    last = temp;
                    j = j + 1;

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
}
