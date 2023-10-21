use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use macroquad::prelude::get_frame_time;

use crate::config::{SPEED_INCREASE_TIMING, SPEED_INCREASE_VALUE, START_GAME_SPEED};
use crate::game::car::{Car, Way};
use crate::game::car::bot_manager::BotManager;
use crate::game::car::player_car::PlayerCar;
use crate::game::graphics::graphics_manager::GraphicsManager;
use crate::game::sounds::sounds_manager::{SoundsManager, SoundType};
use crate::keyboard::GameAction;
use crate::utils::rusty_error::{LockError, RustyError, RustyResult};
use crate::utils::rusty_error::RustyError::RustyLock;
use crate::utils::timer::{Timer, TimerData};

const PLAYER_INPUT_AND_CAR_REACTION: [(GameAction, Way, Way); 4] = [
    (GameAction::Up, Way::Center, Way::Upper),
    (GameAction::Up, Way::Lower, Way::Center),
    (GameAction::Down, Way::Upper, Way::Center),
    (GameAction::Down, Way::Center, Way::Lower),
];

#[derive(Clone, PartialEq, Copy)]
enum GameState {
    NotStarted,
    Running,
    Pause,
    GameOver,
}

pub struct Game {
    receiver_input: Arc<Mutex<Receiver<GameAction>>>,
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
    pub async fn new(receiver_key: Receiver<GameAction>) -> RustyResult<Game> {
        let graphics_manager: GraphicsManager = GraphicsManager::new().await?;
        let mut sounds_manager: SoundsManager = SoundsManager::new().await?;

        let player_car: PlayerCar = PlayerCar::new().await?;

        let start_speed: Arc<Mutex<f32>> = Arc::new(Mutex::new(START_GAME_SPEED));

        // The timer data is used to share the current speed between the timer and the game
        let timer_data: Arc<Mutex<TimerData>> = Arc::new(Mutex::new(
            TimerData::GameSpeed {
                speed: Arc::clone(&start_speed)
            }
        ));

        sounds_manager.play_sound(SoundType::Menu, true);

        Ok(Game {
            receiver_input: Arc::new(Mutex::new(receiver_key)),
            graphics_manager,
            player_car,
            bot_manager: BotManager::new(),
            speed_timer: Timer::new(Game::speed_up, timer_data),
            score: 0,
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

        self.speed_timer.start(SPEED_INCREASE_TIMING);
        self.game_state = GameState::Running;
    }

    pub async fn run(&mut self) -> RustyResult<bool> {
        let game_action: GameAction = self.get_game_action()?;

        // If the game is not running, player can quit the game
        if game_action == GameAction::Quit && self.game_state != GameState::Running {
            return Ok(true);
        }

        if game_action == GameAction::Mute {
            self.sounds_manager.set_mute_songs();
        }

        // The delta time is used to move the background and the bot cars
        let delta_time: f32 = get_frame_time();

        // Here we need to pay attention to the sequence of draw functions due to the
        // superposition of the elements

        let entrance: bool = self.game_state != self.game_previous_state;
        self.game_previous_state = self.game_state;

        match self.game_state {
            GameState::NotStarted => {
                if entrance {
                    self.sounds_manager.play_sound(SoundType::Menu, true);
                }

                if game_action == GameAction::PauseResume {
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

                self.move_player_car(game_action);

                {
                    let current_speed = self.speed.lock().map_err(|e| RustyLock(LockError {
                        message: format!("Impossible to lock the access to the current score: {}", e),
                    }))?;

                    // The score is calculated with the current speed
                    self.score += (0.005 * *current_speed) as u32;

                    // The background and the bot cars are moved with the current speed
                    // But the background is moved with a speed of 80% of the current speed
                    self.graphics_manager.background.set_speed(*current_speed * 0.8);
                    self.graphics_manager.background.move_texture(delta_time);
                    for bot_car in self.bot_manager.bot_car_list.iter_mut() {
                        self.graphics_manager.draw_bot_car(bot_car);
                        bot_car.set_speed(*current_speed);
                    }
                }

                self.graphics_manager.draw_player_car(&self.player_car);
                self.graphics_manager.draw_score(self.score);

                // The player car is colliding with a bot car ?
                let car_colliding = self.manage_bot_cars(delta_time).await?;
                if let Some((way, x_position)) = car_colliding {
                    self.game_state = GameState::GameOver;
                    self.game_over_collision = Some((way, x_position));
                }

                if game_action == GameAction::PauseResume {
                    // No sound in pause menu
                    self.sounds_manager.stop_sound(SoundType::Game);
                    self.game_state = GameState::Pause;
                }
            }
            GameState::Pause => {
                if entrance {
                    // The timer is stopped to avoid to increase the speed while the game is paused
                    self.speed_timer.stop();
                }

                // Draw game situation during the pause

                self.graphics_manager.background.draw();
                self.graphics_manager.draw_score(self.score);

                for bot_car in self.bot_manager.bot_car_list.iter_mut() {
                    self.graphics_manager.draw_bot_car(bot_car);
                }

                self.graphics_manager.draw_player_car(&self.player_car);
                self.graphics_manager.draw_pause(self.session_record);

                if game_action == GameAction::PauseResume {
                    self.game_state = GameState::Running;
                    self.speed_timer.start(SPEED_INCREASE_TIMING);
                }
            }
            GameState::GameOver => {
                if entrance {
                    self.sounds_manager.stop_sound(SoundType::Game);
                    self.sounds_manager.play_sound(SoundType::GameOver, false);
                    self.stop();
                }

                // Draw game situation at the end of the game

                self.graphics_manager.background.draw();

                for bot_car in self.bot_manager.bot_car_list.iter_mut() {
                    self.graphics_manager.draw_bot_car(bot_car);
                }

                self.graphics_manager.draw_player_car(&self.player_car);

                if let Some(collision_position) = self.game_over_collision {
                    self.graphics_manager.draw_collision(collision_position.0, collision_position.1);
                }

                self.graphics_manager.draw_game_over(self.score, self.session_record);

                if game_action == GameAction::PauseResume {
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

    fn stop(&mut self) {
        self.speed_timer.stop();

        if self.session_record < self.score {
            self.session_record = self.score;
        }
    }

    fn move_player_car(&mut self, game_action: GameAction) {
        // Get the new way if the player car can move
        if let Some(new_way) = Game::get_destination_way(game_action, self.player_car.get_way()) {
            self.player_car.set_way(new_way);
        }
    }

    fn get_game_action(&self) -> RustyResult<GameAction> {
        let receiver_input = self.receiver_input.lock().map_err(|e| RustyLock(LockError {
            message: format!("Impossible to lock the access to the player car: {}", e),
        }))?;

        match receiver_input.try_recv() {
            Ok(key) => Ok(key),
            Err(e) => {
                if e == std::sync::mpsc::TryRecvError::Empty {
                    Ok(GameAction::None)
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
            Ok(mut speed) => {
                *speed += SPEED_INCREASE_VALUE;
            }
            Err(e) => {
                println!("Error lock current speed: {}", e);
            }
        };
    }

    fn get_destination_way(game_action: GameAction, way: Way) -> Option<Way> {
        let mut destination_way: Option<Way> = None;
        for &(k, w1, w2) in &PLAYER_INPUT_AND_CAR_REACTION {
            if k == game_action && w1 == way {
                destination_way = Some(w2);
                break;
            }
        }
        return destination_way;
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