use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::thread;

use macroquad::input::KeyCode;

use crate::game::car::bot_car::BotCar;
use crate::game::car::player_car::PlayerCar;
use crate::game::car::Way;
use crate::game::graphics::graphics_manager::GraphicsManager;
use crate::utils::timer::{Timer, TimerData};

pub struct Game {
    receiver_input: Arc<Mutex<Receiver<KeyCode>>>,
    running: Arc<AtomicBool<>>,
    graphics_manager: GraphicsManager,
    pub player_car: Arc<Mutex<PlayerCar>>,
    bot_cars: Vec<BotCar>,
    game_timer: Timer,
    pub score: Arc<Mutex<u32>>,
}

impl Game {
    pub async fn new(receiver_key: Receiver<KeyCode>) -> Option<Game> {
        let graphics_manager = GraphicsManager::new().await;
        let player_car = match PlayerCar::new().await {
            Some(pc) => Arc::new(Mutex::new(pc)),
            None => return None, // Return early if PlayerCar creation fails.
        };

        let score = Arc::new(Mutex::new(0));
        let score_lock = score.lock();

        let _unused = match score_lock {
            Ok(init_score) => init_score,
            Err(e) => {
                println!("Error lock current score: {}", e);
                return None;
            }
        };

        graphics_manager.map(| graphics_manager | Game {
            receiver_input: Arc::new(Mutex::new(receiver_key)),
            running: Arc::new(AtomicBool::new(true)),
            graphics_manager,
            player_car,
            bot_cars: Vec::new(),
            game_timer: Timer::new(Game::add_score, Arc::new(Mutex::new(TimerData::GameScore { score: Arc::clone(&score) }))),
            score: Arc::clone(&score),
        })
    }

    fn add_score(timer_data: &mut TimerData) {
        let TimerData::GameScore { score: game_score } = timer_data;
        let game_score_lock = game_score.lock();

        match game_score_lock {
            Ok(mut init_score) => *init_score += 1,
            Err(e) => {
                println!("Error lock current score: {}", e);
            }
        };
    }

    pub fn start(&mut self) {
        let receiver_input_clone = Arc::clone(&self.receiver_input);
        let running_clone = Arc::clone(&self.running);
        let player_car_clone = Arc::clone(&self.player_car);

        self.game_timer.start(500);

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