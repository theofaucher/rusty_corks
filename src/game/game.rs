use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::thread;

use macroquad::input::KeyCode;
use macroquad::prelude::get_frame_time;

use crate::game::car::bot_car::BotCar;
use crate::game::car::player_car::PlayerCar;
use crate::game::car::Way;
use crate::game::graphics::graphics_manager::GraphicsManager;
use crate::utils::rusty_error::{RustyError, RustyResult};
use crate::utils::timer::{Timer, TimerData};

#[derive(Clone)]
enum GameState {
    NotStarted,
    Running,
    GameOver,
}

#[derive(Clone)]
pub struct Game {
    receiver_input: Arc<Mutex<Receiver<KeyCode>>>,
    running: Arc<AtomicBool<>>,
    graphics_manager: GraphicsManager,
    pub player_car: Arc<Mutex<PlayerCar>>,
    bot_cars: Vec<BotCar>,
    game_timer: Timer,
    pub score: Arc<Mutex<u32>>,
    game_state: GameState,
}

impl Game {
    pub async fn new(receiver_key: Receiver<KeyCode>) -> RustyResult<Game> {
        let graphics_manager = GraphicsManager::new().await?;

        let player_car = PlayerCar::new().await?;

        let score = Arc::new(Mutex::new(0));

        Ok(Game {
            receiver_input: Arc::new(Mutex::new(receiver_key)),
            running: Arc::new(AtomicBool::new(true)),
            graphics_manager,
            player_car: Arc::new(Mutex::new(player_car)),
            bot_cars: Vec::new(),
            game_timer: Timer::new(Game::add_score, Arc::new(Mutex::new(TimerData::GameScore { score: Arc::clone(&score) }))),
            score: Arc::clone(&score),
            game_state: GameState::NotStarted,
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
        let game_clone = self.clone();
        self.game_timer.start(500);

        thread::spawn(move || {
            game_clone.move_player_car();
        });

        self.game_state = GameState::Running;
    }

    fn get_keyboard_input(&self) -> RustyResult<KeyCode> {
        let player_input = self.receiver_input.lock();

        if let Ok(receiver_input) = player_input {
            match receiver_input.try_recv() {
                Ok(key) => Ok(key),
                Err(e) => {
                    if e == std::sync::mpsc::TryRecvError::Empty {
                        Ok(KeyCode::Unknown)
                    } else {
                        Err(RustyError::Recv(e))
                    }
                }
            }
        } else {
            Err(RustyError::RustyLock)
        }
    }

    pub async fn run(&mut self) -> RustyResult<()> {
        let player_input = self.get_keyboard_input()?;

        let delta_time = get_frame_time();

        self.graphics_manager.background.update_position(delta_time);

        match self.game_state {
            GameState::NotStarted => {
                if player_input == KeyCode::Enter {
                    self.start();
                } else {
                    self.graphics_manager.draw_new_game();
                }
            }
            GameState::Running => {
                let player_car = self.player_car.lock();
                let player_car = match player_car {
                    Ok(player_car) => player_car,
                    Err(_) => Err(RustyError::RustyLock)?,
                };

                self.graphics_manager.draw_player_car(player_car);

                let current_score = self.score.lock();
                let current_score = match current_score {
                    Ok(current_score) => current_score,
                    Err(_) => Err(RustyError::RustyLock)?,
                };

                self.graphics_manager.draw_score(*current_score);
            }
            GameState::GameOver => {
                println!("Game over");
            }
        }

        Ok(())
    }

    pub fn move_player_car(&self) {
        let mut player_input_and_car_reaction: HashMap<(usize, Way), Way> = HashMap::new();

        player_input_and_car_reaction.insert((KeyCode::Up as usize, Way::Center), Way::Upper);
        player_input_and_car_reaction.insert((KeyCode::Up as usize, Way::Lower), Way::Center);
        player_input_and_car_reaction.insert((KeyCode::Down as usize, Way::Upper), Way::Center);
        player_input_and_car_reaction.insert((KeyCode::Down as usize, Way::Center), Way::Lower);


        while self.running.load(Ordering::Relaxed) {
            let player_input = self.get_keyboard_input();
            let player_input = match player_input {
                Ok(player_input) => player_input as usize,
                Err(e) => {
                    println!("Error getting player input: {}", e);
                    break;
                }
            };

            let player_car_lock = self.player_car.lock();

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