use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::sync::mpsc::{Sender};
use macroquad::input::{is_key_down, KeyCode};
use std::time::{Duration, Instant};
use crate::keyboard_observer::key_game::KeyGame;

mod key_game;

pub struct KeyboardObserver {
    sender: Arc<Mutex<Sender<KeyCode>>>,
    running: Arc<AtomicBool<>>,
}

impl KeyboardObserver {
    pub fn new(sender_key: Sender<KeyCode>) -> KeyboardObserver {
        KeyboardObserver {
            sender: Arc::new(Mutex::new(sender_key)),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    fn observer(sender: &Arc<Mutex<Sender<KeyCode>>>, keys_games: &mut [KeyGame]) {
        let mut key_pressed: Option<KeyCode> = None;
        for key_games in keys_games.iter() {
            if key_games.is_key_pressed(){
                key_pressed = Some(key_games.key);
                break;
            }

        }

        if let Some(key) = key_pressed {
            let send_status_lock = sender.lock();

            match send_status_lock {
                Ok(send_status) => {
                    let send_status = send_status.send(key);
                    if let Err(e) = send_status {
                        println!("Error sending key: {}", e);
                    }
                },
                Err(e) => {
                    println!("Error sending key: {}", e);
                }
            }
        }
    }

    pub fn start_observer(&self) -> JoinHandle<()> {
        let sender_clone = Arc::clone(&self.sender);
        let running_clone = Arc::clone(&self.running);

        thread::spawn(move || {
            let mut keys_games: Vec<KeyGame> = Vec::new();
            let key_game = [KeyCode::Down, KeyCode::Up];

            for key in key_game.iter() {
                keys_games.push(KeyGame::new(*key));
            }

            let timer_duration = Duration::from_millis(20);
            let mut last_time = Instant::now();


            while running_clone.load(Ordering::Relaxed) {
                for key_games in &mut keys_games {
                    let is_down = is_key_down(key_games.key);
                    key_games.update(is_down);
                }

                Self::observer(&sender_clone, &mut keys_games);

                let elapsed_time = last_time.elapsed();

                if elapsed_time < timer_duration {
                    let sleep_time = timer_duration - elapsed_time;
                    sleep(sleep_time);
                }

                last_time = Instant::now();
            }
        })
    }

    #[warn(dead_code)]
    pub fn stop_observer(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}