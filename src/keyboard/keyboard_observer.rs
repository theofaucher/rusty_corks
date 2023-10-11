use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};

use macroquad::input::{is_key_down, KeyCode};

use crate::keyboard::key_game::KeyGame;

const KEY_GAME: [KeyCode; 3] = [KeyCode::Down, KeyCode::Up, KeyCode::Enter];

#[derive(Clone)]
pub struct KeyboardObserver {
    sender: Arc<Mutex<Sender<KeyCode>>>,
    keys_games: Vec<KeyGame>,
    pub running: Arc<AtomicBool<>>,
}

impl KeyboardObserver {
    pub fn new(sender_key: Sender<KeyCode>) -> KeyboardObserver {
        let mut keys_games = Vec::new();
        for key in KEY_GAME.iter() {
            keys_games.push(KeyGame::new(*key));
        }

        KeyboardObserver {
            sender: Arc::new(Mutex::new(sender_key)),
            keys_games,
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    fn observer(&self) {
        let mut key_pressed: Option<KeyCode> = None;
        for key_games in self.keys_games.iter() {
            if key_games.is_key_pressed(){
                key_pressed = Some(key_games.key);
                break;
            }

        }

        if let Some(key) = key_pressed {
            let send_status_lock = self.sender.lock();

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
        let mut observer_clone = self.clone();

        thread::spawn(move || {
            let timer_duration = Duration::from_millis(20);
            let mut last_time = Instant::now();


            while observer_clone.running.load(Ordering::Relaxed) {
                for key_games in &mut observer_clone.keys_games {
                    let is_down = is_key_down(key_games.key);
                    key_games.update(is_down);
                }

                observer_clone.observer();

                let elapsed_time = last_time.elapsed();

                if elapsed_time < timer_duration {
                    let sleep_time = timer_duration - elapsed_time;
                    sleep(sleep_time);
                }

                last_time = Instant::now();
            }
            println!("Stop observer");
        })
    }

    pub fn stop_observer(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}