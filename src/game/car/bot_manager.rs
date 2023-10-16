use std::sync::{Arc, Mutex};
use std::time::Instant;

use rand::Rng;

use crate::game::car::bot_car::BotCar;
use crate::game::car::Way;
use crate::utils::rusty_error::{LockError, RustyResult};
use crate::utils::rusty_error::RustyError::{LaneNotFound, RustyLock};

pub struct BotManager {
    pub bot_car_list: Vec<BotCar>,
    game_speed: Arc<Mutex<f32>>,
    lanes: [Lane; 3],
}

#[derive(PartialEq)]
struct Lane {
    way: Way,
    last_spawn_time: Instant,
}

impl BotManager {
    pub fn new(game_speed: Arc<Mutex<f32>>) -> BotManager {
        BotManager {
            bot_car_list: Vec::new(),
            lanes: [
                Lane {
                    way: Way::Upper,
                    last_spawn_time: Instant::now(),
                },
                Lane {
                    way: Way::Center,
                    last_spawn_time: Instant::now(),
                },
                Lane {
                    way: Way::Lower,
                    last_spawn_time: Instant::now(),
                },
            ],
            game_speed,
        }
    }
    pub async fn spawn_car(&mut self) -> RustyResult<()> {
        let mut rng = rand::thread_rng();
        let way_idx = rng.gen_range(0..self.lanes.len());
        let way = match way_idx {
            0 => Way::Upper,
            1 => Way::Center,
            _ => Way::Lower,
        };
        let lane_pos = self.lanes.iter().position(|l| l.way == way).ok_or(LaneNotFound)?;
        let lane = &self.lanes[lane_pos];
        if self.is_lane_free(lane)? {
            // Créez un nouvelle voiture et ajoutez-le à la voie
            let bot_car = BotCar::new(lane.way, Arc::clone(&self.game_speed)).await?;
            self.bot_car_list.push(bot_car);

            self.lanes[way_idx].last_spawn_time = Instant::now();
        }
        Ok(())
    }
    fn is_lane_recently_used(&self, lane: &Lane) -> RustyResult<bool> {
        // Vérifiez si la voie a été utilisée récemment en vérifiant
        // le temps écoulé depuis la dernière apparition d'un objet dans la voie.
        let now = Instant::now();
        let elapsed_time = now.duration_since(lane.last_spawn_time);
        let current_speed = self.game_speed.lock().map_err(|e| RustyLock(LockError {
            message: format!("Impossible to lock the access to the current speed: {}", e),
        }))?;
        let ret = elapsed_time.as_millis() < ((4600.0 / *current_speed) * 100.0) as u128;
        Ok(ret)
    }
    fn is_lane_free(&self, lane: &Lane) -> RustyResult<bool> {
        let ret;
        if self.bot_car_list.is_empty() {
            ret = true;
        } else if self.is_lane_recently_used(lane)? {
            ret = false;
        } else {
            let lane_position = match self.lanes.iter().position(|l| l == lane) {
                Some(i) => i,
                None => unreachable!(),
            };

            if lane_position == 0 {
                ret = !self.is_lane_recently_used(&self.lanes[lane_position + 1])?
            } else if lane_position == self.lanes.len() - 1 {
                ret = !self.is_lane_recently_used(&self.lanes[lane_position - 1])?
            } else {
                ret = !(self.is_lane_recently_used(&self.lanes[lane_position - 1])? && self.is_lane_recently_used(&self.lanes[lane_position + 1])?)
            }
        }
        Ok(ret)
    }
}