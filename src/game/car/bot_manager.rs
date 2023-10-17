use macroquad::prelude::screen_width;
use rand::Rng;

use crate::game::car::bot_car::BotCar;
use crate::game::car::Car;
use crate::game::car::Way;
use crate::game::game::DISTANCE_BETWEEN_CARS;
use crate::utils::rusty_error::RustyError::LaneNotFound;
use crate::utils::rusty_error::RustyResult;

pub struct BotManager {
    pub bot_car_list: Vec<BotCar>,
    lanes: [Lane; 3],
}

#[derive(PartialEq)]
struct Lane {
    way: Way,
}

impl BotManager {
    pub fn new() -> BotManager {
        BotManager {
            bot_car_list: Vec::new(),
            lanes: [
                Lane {
                    way: Way::Upper,
                },
                Lane {
                    way: Way::Center,
                },
                Lane {
                    way: Way::Lower,
                },
            ],
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
            let bot_car = BotCar::new(lane.way).await?;
            self.bot_car_list.push(bot_car);

        }
        Ok(())
    }
    fn is_lane_recently_used(&self, lane: &Lane) -> RustyResult<bool> {
        // Vérifiez si la voie a été utilisée récemment en vérifiant
        // le temps écoulé depuis la dernière apparition d'un objet dans la voie.

        let mut recently_used = false;

        for bot_car in &self.bot_car_list {
            if (screen_width() - bot_car.x_position) < (screen_width() / DISTANCE_BETWEEN_CARS) && bot_car.get_way() == lane.way {
                recently_used = true;
                break;
            }
        }
        Ok(recently_used)
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