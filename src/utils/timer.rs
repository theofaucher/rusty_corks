use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

type TimerCallback = fn(timer_data: &mut TimerData);

#[derive(Clone)] // Ajoutez cette dérivation pour le trait Copy
pub enum TimerData {
    GameScore { score: Arc<Mutex<u32>> },
}

#[derive(Clone)]
pub struct Timer {
    callback: TimerCallback,
    timer_data: Arc<Mutex<TimerData>>,
    running: Arc<AtomicBool>,
}

impl Timer {
    pub fn new(callback: TimerCallback, timer_data: Arc<Mutex<TimerData>>) -> Timer {
        Timer {
            callback,
            timer_data,
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start(&mut self, delay: u16) {
        let running = Arc::clone(&self.running);
        let timer_data = Arc::clone(&self.timer_data);
        let callback = self.callback;

        thread::spawn(move || {
            let timer_duration = Duration::from_millis(delay as u64);
            let mut last_time = Instant::now();

            while running.load(Ordering::Relaxed) {
                let timer_data = timer_data.lock();

                match timer_data {
                    Ok(mut timer_data) => {
                        callback(&mut timer_data);
                    }
                    Err(e) => {
                        println!("Error lock timer_data: {}", e);
                    }
                }

                let elapsed_time = last_time.elapsed();

                if elapsed_time < timer_duration {
                    let sleep_time = timer_duration - elapsed_time;
                    sleep(sleep_time);
                }

                last_time = Instant::now();
            }
        });
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}