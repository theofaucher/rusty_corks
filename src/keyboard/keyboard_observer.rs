use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};

use macroquad::input::{is_key_down, KeyCode};

use crate::config::KEY_GAME;
use crate::keyboard::{GameAction, get_game_action_from_key_code};
use crate::keyboard::key_game::KeyGame;

pub struct KeyboardObserver {
    sender: Sender<GameAction>,
    keys_games: Vec<KeyGame>,
    pub running: Arc<AtomicBool<>>,
    thread: Option<JoinHandle<()>>,
}

impl KeyboardObserver {
    pub fn new(sender_key: Sender<GameAction>) -> KeyboardObserver {
        let mut keys_games = Vec::new();
        for key in KEY_GAME.iter() {
            keys_games.push(KeyGame::new(key.0));
        }

        KeyboardObserver {
            sender: sender_key,
            keys_games,
            running: Arc::new(AtomicBool::new(true)),
            thread: None,
        }
    }

    fn observer(keys_games: &[KeyGame], sender: &Sender<GameAction>) {
        let mut key_pressed: Option<KeyCode> = None;
        for key_games in keys_games.iter() {
            if key_games.is_key_pressed() {
                key_pressed = Some(key_games.key);
                break;
            }
        }

        if let Some(key) = key_pressed {
            // Convert key to game action and send it in the channel
            let game_action = get_game_action_from_key_code(key);
            if let Some(game_action) = game_action {
                let send_status = sender.send(game_action);
                if let Err(e) = send_status {
                    // If a problem occurs, print the error and continue
                    println!("Error sending key: {}", e);
                }
            }
        }
    }

    pub fn start_observer(&mut self) {
        let sender_clone = self.sender.clone();
        let running_clone = Arc::clone(&self.running);
        let mut keys_games_clone = self.keys_games.clone();

        self.thread = Some(thread::spawn(move || {
            let timer_duration = Duration::from_millis(20);
            let mut last_time = Instant::now();


            while running_clone.load(Ordering::Relaxed) {
                // Detect if a key is pressed (falling edge)
                for key_game in keys_games_clone.iter_mut() {
                    let is_down = is_key_down(key_game.key);
                    key_game.update(is_down);
                }

                // If a key is pressed (one per loop iteration), send it in the channel
                KeyboardObserver::observer(&keys_games_clone, &sender_clone);

                let elapsed_time = last_time.elapsed();

                if elapsed_time < timer_duration {
                    // Compute the remaining time to sleep
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