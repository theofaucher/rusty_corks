use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use macroquad::input::KeyCode;
use crate::game::car::bot_car::BotCar;
use crate::game::car::player_car::PlayerCar;
use crate::game::car::Way;
use crate::game::graphics::graphics_manager::GraphicsManager;


pub struct Game {
    receiver_input: Arc<Mutex<Receiver<KeyCode>>>,
    running: Arc<AtomicBool<>>,
    graphics_manager: GraphicsManager,
    pub player_car: Arc<Mutex<PlayerCar>>,
    bot_cars: Vec<BotCar>,
    score: u32,
}

impl Game {
    pub async fn new(receiver_key: Receiver<KeyCode>) -> Option<Game> {
        let graphics_manager = GraphicsManager::new().await;
        let player_car = match PlayerCar::new().await {
            Some(pc) => Arc::new(Mutex::new(pc)),
            None => return None, // Return early if PlayerCar creation fails.
        };

        graphics_manager.map(|graphics_manager| Game {
            receiver_input: Arc::new(Mutex::new(receiver_key)),
            running: Arc::new(AtomicBool::new(true)),
            graphics_manager,
            player_car,
            bot_cars: Vec::new(),
            score: 0,
        })
    }

    pub fn start(&mut self) {
        let receiver_input_clone = Arc::clone(&self.receiver_input);
        let running_clone = Arc::clone(&self.running);
        let player_car_clone = Arc::clone(&self.player_car);

        thread::spawn(move || {
            Game::move_player_car(receiver_input_clone, running_clone, player_car_clone);
        });
    }

    pub fn move_player_car(receiver_input: Arc<Mutex<Receiver<KeyCode>>>, running: Arc<AtomicBool<>>, player_car: Arc<Mutex<PlayerCar>>) {
        let mut player_input_and_car_reaction: HashMap<(usize, Way), Way> = HashMap::new();

        player_input_and_car_reaction.insert((KeyCode::Up as usize, Way::Center), Way::Upper);
        player_input_and_car_reaction.insert((KeyCode::Up as usize, Way::Lower), Way::Center);
        player_input_and_car_reaction.insert((KeyCode::Down as usize, Way::Upper), Way::Center);
        player_input_and_car_reaction.insert((KeyCode::Down as usize, Way::Center), Way::Lower);


        while running.load(Ordering::Relaxed) {
            let receiver_input_lock = receiver_input.lock();

            let player_input = match receiver_input_lock {
                Ok(receiver_input) => {
                    match receiver_input.recv() {
                        Ok(key) => key as usize,
                        Err(e) => {
                            println!("Error receiving key: {}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("Error receiving key: {}", e);
                    break;
                }
            };

            let player_car_lock = player_car.lock();

            let mut player_car = match player_car_lock {
                Ok(player_car) => player_car,
                Err(e) => {
                    println!("Error locking player car: {}", e);
                    break;
                }
            };

            if let Some(new_way) = player_input_and_car_reaction.get(&(player_input, player_car.way)) {
                player_car.way = *new_way;
            }
        }
    }
}