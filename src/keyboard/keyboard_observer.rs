use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};

use macroquad::input::{is_key_down, KeyCode};

use crate::keyboard::key_game::KeyGame;

const KEY_GAME: [KeyCode; 4] = [KeyCode::Down, KeyCode::Up, KeyCode::Enter, KeyCode::Escape];

pub struct KeyboardObserver {
    sender: Arc<Mutex<Sender<KeyCode>>>,
    keys_games: Vec<KeyGame>,
    pub running: Arc<AtomicBool<>>,
    thread: Option<JoinHandle<()>>,
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
            thread: None,
        }
    }

    fn observer(keys_games: &[KeyGame], sender: &Arc<Mutex<Sender<KeyCode>>>) {
        let mut key_pressed: Option<KeyCode> = None;
        for key_games in keys_games.iter() {
            if key_games.is_key_pressed() {
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
                }
                Err(e) => {
                    println!("Error sending key: {}", e);
                }
            }
        }
    }

    pub fn start_observer(&mut self) {
        let sender_clone = Arc::clone(&self.sender);
        let running_clone = Arc::clone(&self.running);
        let mut keys_games_clone = self.keys_games.clone();

        self.thread = Some(thread::spawn(move || {
            let timer_duration = Duration::from_millis(20);
            let mut last_time = Instant::now();


            while running_clone.load(Ordering::Relaxed) {
                for key_game in keys_games_clone.iter_mut() {
                    let is_down = is_key_down(key_game.key);
                    key_game.update(is_down);
                }

                KeyboardObserver::observer(&keys_games_clone, &sender_clone);

                let elapsed_time = last_time.elapsed();

                if elapsed_time < timer_duration {
                    let sleep_time = timer_duration - elapsed_time;
                    sleep(sleep_time);
                }

                last_time = Instant::now();
            }
        }));
    }

    pub fn stop_observer(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if self.thread.is_some() {
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }
}