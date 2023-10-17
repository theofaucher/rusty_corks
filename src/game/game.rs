use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use macroquad::input::KeyCode;
use macroquad::prelude::get_frame_time;

use crate::config::START_GAME_SPEED;
use crate::game::car::{Car, Way};
use crate::game::car::bot_manager::BotManager;
use crate::game::car::player_car::PlayerCar;
use crate::game::graphics::graphics_manager::GraphicsManager;
use crate::game::sounds::sounds_manager::{SoundsManager, SoundType};
use crate::utils::rusty_error::{LockError, RustyError, RustyResult};
use crate::utils::rusty_error::RustyError::RustyLock;
use crate::utils::timer::{Timer, TimerData};

const PLAYER_INPUT_AND_CAR_REACTION: [(usize, Way, Way); 4] = [
    (KeyCode::Z as usize, Way::Center, Way::Upper),
    (KeyCode::Z as usize, Way::Lower, Way::Center),
    (KeyCode::S as usize, Way::Upper, Way::Center),
    (KeyCode::S as usize, Way::Center, Way::Lower),
];

#[derive(Clone, PartialEq, Copy)]
enum GameState {
    NotStarted,
    Running,
    Pause,
    GameOver,
}

pub struct Game {
    receiver_input: Arc<Mutex<Receiver<KeyCode>>>,
    graphics_manager: GraphicsManager,
    pub player_car: PlayerCar,
    bot_manager: BotManager,
    speed_timer: Timer,
    score: u32,
    session_record: u32,
    speed: Arc<Mutex<f32>>,
    game_state: GameState,
    game_previous_state: GameState,
    game_over_collision: Option<(Way, f32)>,
    sounds_manager: SoundsManager,
}

impl Game {
    pub async fn new(receiver_key: Receiver<KeyCode>) -> RustyResult<Game> {
        let graphics_manager = GraphicsManager::new().await?;
        let mut sounds_manager = SoundsManager::new().await?;

        let player_car = PlayerCar::new().await?;

        let score = 0;

        let start_speed = Arc::new(Mutex::new(START_GAME_SPEED));

        sounds_manager.play_sound(SoundType::Menu, true);

        Ok(Game {
            receiver_input: Arc::new(Mutex::new(receiver_key)),
            graphics_manager,
            player_car,
            bot_manager: BotManager::new(),
            speed_timer: Timer::new(Game::speed_up, Arc::new(Mutex::new(TimerData::GameSpeed { speed: Arc::clone(&start_speed) }))),
            score,
            session_record: 0,
            speed: Arc::clone(&start_speed),
            game_state: GameState::NotStarted,
            game_previous_state: GameState::NotStarted,
            game_over_collision: None,
            sounds_manager,
        })
    }

    pub fn start(&mut self) {
        self.score = 0;

        self.speed_timer.start(100);
        self.game_state = GameState::Running;
    }

    pub async fn run(&mut self) -> RustyResult<bool> {
        let player_input = self.get_keyboard_input()?;

        if player_input == KeyCode::Escape && self.game_state != GameState::Running {
            return Ok(true);
        }

        if player_input == KeyCode::M {
            self.sounds_manager.set_mute_songs();
        }

        let delta_time = get_frame_time();

        let entrance: bool = self.game_state != self.game_previous_state;
        self.game_previous_state = self.game_state;

        match self.game_state {
            GameState::NotStarted => {
                if entrance {
                    self.sounds_manager.play_sound(SoundType::Menu, true);
                }

                if player_input == KeyCode::Space {
                    self.sounds_manager.stop_sound(SoundType::Menu);
                    self.start();
                } else {
                    self.graphics_manager.background.move_texture(delta_time);
                    self.graphics_manager.draw_new_game();
                }
            }
            GameState::Running => {
                if entrance {
                    self.sounds_manager.play_sound(SoundType::Game, true);
                }

                self.move_player_car(player_input)?;

                {
                    let current_speed = self.speed.lock().map_err(|e| RustyLock(LockError {
                        message: format!("Impossible to lock the access to the current score: {}", e),
                    }))?;

                    self.score += (0.005 * *current_speed) as u32;

                    self.graphics_manager.background.set_speed(*current_speed * 0.8);
                    self.graphics_manager.background.move_texture(delta_time);
                    for bot_car in self.bot_manager.bot_car_list.iter_mut() {
                        self.graphics_manager.draw_bot_car(bot_car);
                        bot_car.set_speed(*current_speed);
                    }
                }

                self.graphics_manager.draw_player_car(&self.player_car);
                self.graphics_manager.draw_score(self.score);

                let car_colliding = self.manage_bot_cars(delta_time).await?;
                if let Some((way, x_position)) = car_colliding {
                    self.game_state = GameState::GameOver;
                    self.game_over_collision = Some((way, x_position));
                }

                if player_input == KeyCode::Space {
                    self.sounds_manager.stop_sound(SoundType::Game);
                    self.game_state = GameState::Pause;
                }
            }
            GameState::Pause => {
                if entrance {
                    self.speed_timer.stop();
                }

                self.graphics_manager.background.draw();
                self.graphics_manager.draw_score(self.score);

                for bot_car in self.bot_manager.bot_car_list.iter_mut() {
                    self.graphics_manager.draw_bot_car(bot_car);
                }

                self.graphics_manager.draw_player_car(&self.player_car);
                self.graphics_manager.draw_pause(self.session_record);

                if player_input == KeyCode::Space {
                    self.game_state = GameState::Running;
                    self.speed_timer.start(100);
                }
            }
            GameState::GameOver => {
                if entrance {
                    self.sounds_manager.stop_sound(SoundType::Game);
                    self.sounds_manager.play_sound(SoundType::GameOver, false);
                    self.stop()?;
                }

                self.graphics_manager.background.draw();

                for bot_car in self.bot_manager.bot_car_list.iter_mut() {
                    self.graphics_manager.draw_bot_car(bot_car);
                }

                self.graphics_manager.draw_player_car(&self.player_car);

                if let Some(collision_position) = self.game_over_collision {
                    self.graphics_manager.draw_collision(collision_position.0, collision_position.1);
                }

                self.graphics_manager.draw_game_over(self.score, self.session_record);

                if player_input == KeyCode::Space {
                    self.game_state = GameState::NotStarted;

                    self.sounds_manager.stop_sound(SoundType::GameOver);

                    let mut current_speed = self.speed.lock().map_err(|e| RustyLock(LockError {
                        message: format!("Impossible to lock the access to the current score: {}", e),
                    }))?;

                    *current_speed = START_GAME_SPEED;
                    self.graphics_manager.background.set_speed(*current_speed);

                    self.bot_manager.bot_car_list.clear();
                }
            }
        }

        Ok(false)
    }

    fn stop(&mut self) -> RustyResult<()> {
        self.speed_timer.stop();

        if self.session_record < self.score {
            self.session_record = self.score;
        }

        Ok(())
    }

    fn move_player_car(&mut self, player_input: KeyCode) -> RustyResult<()> {
        if let Some(new_way) = Game::get_destination_way(player_input as usize, self.player_car.get_way()) {
            self.player_car.set_way(new_way);
        }

        Ok(())
    }

    fn get_keyboard_input(&self) -> RustyResult<KeyCode> {
        let receiver_input = self.receiver_input.lock().map_err(|e| RustyLock(LockError {
            message: format!("Impossible to lock the access to the player car: {}", e),
        }))?;

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
    }

    fn speed_up(timer_data: &mut TimerData) {
        let TimerData::GameSpeed { speed: game_speed } = timer_data;
        let game_speed_lock = game_speed.lock();

        match game_speed_lock {
            Ok(mut init_speed) => {
                *init_speed += 1.0;
            },
            Err(e) => {
                println!("Error lock current speed: {}", e);
            }
        };
    }

    fn get_destination_way(key_code: usize, way: Way) -> Option<Way> {
        let mut destination_way: Option<Way> = None;
        for &(k, w1, w2) in &PLAYER_INPUT_AND_CAR_REACTION {
            if k == key_code && w1 == way {
                destination_way = Some(w2);
                break;
            }
        }
        destination_way
    }

    async fn manage_bot_cars(&mut self, delta_time: f32) -> RustyResult<Option<(Way, f32)>> {
        self.bot_manager.spawn_car().await?;
        let mut is_colliding: Option<(Way, f32)> = None;

        for bot_car in self.bot_manager.bot_car_list.iter_mut() {
            bot_car.update_position(delta_time);
            is_colliding = bot_car.is_colliding(&self.player_car);
            if is_colliding.is_some() {
                break;
            }
        }

        self.bot_manager.bot_car_list.retain(|bot_car| {
            !bot_car.is_out_of_screen()
        });

        Ok(is_colliding)
    }
}