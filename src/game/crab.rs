use ggez::{Context, GameResult};
use ggez::graphics;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{CRAB_W,
                          CRAB_S};

pub struct Crab {
    pub location: Point2,
    velocity: Vector2,
    w: f32,
    s: f32
}

impl Crab {
    pub fn new(location: Point2) -> GameResult<Crab> {
        let c = Crab {
            location,
            velocity: Vector2::new(CRAB_S, 0.0),
            w: CRAB_W,
            s: CRAB_S
        };
        Ok(c)
    }

    pub fn update(&mut self, max_screen: f32) -> GameResult<&Self> {
        /*
        * TODO: Move crab left to right
        */
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: Draw crab image
        */
        Ok(self)
    }
}
