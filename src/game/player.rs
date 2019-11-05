use ggez::{Context, GameResult};
use ggez::graphics;
use nalgebra as na;

use crate::game::claw::{Claw, Directions};

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;


pub struct Player {
    pub score: usize,
    pub claw: Claw
}

impl Player {
    pub fn new(loc: Point2,
               b_anchor: Vector2,
               j_anchor: Vector2) -> GameResult<Player> {
        let p = Player {
            score: 0,
            claw: Claw::new(loc,
                            b_anchor,
                            j_anchor)?
        };
        Ok(p)
    }

    pub fn update(&mut self, new_loc: Point2) -> GameResult<&Self> {
        /*
        * TODO: Update claw
        */
        Ok(self)
    }

    pub fn draw(&self,
                ctx: &mut Context,
                img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: Draw claw
        */
        Ok(self)
    }

    pub fn increase_score(&mut self) -> GameResult<&Self> {
        /*
        * TODO: Make score go up
        */
        Ok(self)
    }

    pub fn movedir(&mut self, dir: Directions) {
        /*
        * TODO: Move the claw
        */
    }
}
