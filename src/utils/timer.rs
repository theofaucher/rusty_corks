use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};

// Pointer to a function used to be a "callback" function, call by the timer
type TimerCallback = fn(timer_data: &mut TimerData);

// Data used by the callback
#[derive(Clone)]
pub enum TimerData {
    GameSpeed { speed: Arc<Mutex<f32>> },
}

pub struct Timer {
    callback: TimerCallback,
    timer_data: Arc<Mutex<TimerData>>,
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Timer {
    pub fn new(callback: TimerCallback, timer_data: Arc<Mutex<TimerData>>) -> Timer {
        Timer {
            callback,
            timer_data,
            running: Arc::new(AtomicBool::new(true)),
            thread: None,
        }
    }

    fn run(running: Arc<AtomicBool>, timer_data: Arc<Mutex<TimerData>>, delay: u16, callback: TimerCallback) {
        let mut timer_duration = Duration::from_millis(delay as u64);

        let mut last_time: Instant = Instant::now();

        while running.load(Ordering::Relaxed) {
            let elapsed_time = last_time.elapsed();

            // If the elapsed time is greater than the timer duration, we call the callback function
            if elapsed_time > timer_duration {
                let execution_time: Instant = Instant::now();

                let timer_data = timer_data.lock();
                match timer_data {
                    Ok(mut timer_data) => {
                        callback(&mut timer_data);
                    }
                    Err(e) => {
                        println!("Error lock timer_data: {}", e);
                    }
                }

                let execution_time = execution_time.elapsed();
                timer_duration = Duration::from_millis(delay as u64) - execution_time;

                last_time = Instant::now();
            }
            // Sleep 1ms to avoid using too much CPU resources
            sleep(Duration::from_millis(1));
        }
    }


    pub fn start(&mut self, delay: u16) {
        let running_clone = Arc::clone(&self.running);
        let timer_data_clone = Arc::clone(&self.timer_data);
        let callback = self.callback;

        self.running.store(true, Ordering::Relaxed);

        self.thread = Some(thread::spawn(move || {
            Timer::run(running_clone, timer_data_clone, delay, callback);
        }));
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if self.thread.is_some() {
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }
}