use ggez::{Context, GameResult};
use ggez::graphics;
use rand;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{SCREEN_H, SCREEN_W, SNACK_W};

pub struct Snack {
    location: Point2,
    velocity: Vector2,
    w: f32,
    active: bool
}

impl Snack {
    fn new () -> GameResult<Snack> {
        let s = Snack {
            location: Point2::new(rand::random::<f32>() * SCREEN_W,
                                  rand::random::<f32>() * SCREEN_H - SCREEN_H),
            velocity: Vector2::new(0.0,
                                   rand::random::<f32>() * 2.0 + 0.1),
            w: SNACK_W,
            active: true
        };
        Ok(s)
    }

    pub fn update(&mut self) -> GameResult<&Self> {
        /*
        * TODO: 
        * 1. Move snack down
        * 2. Set active to false if the snack has left the screen
        * 3. If not active, reset the snack
        */
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: 
        * Draw the snack, but only if it's active
        */
        Ok(self)
    }

    pub fn collides_with(&mut self, other: Point2) -> bool {
        /*
        * TODO: 
        * Check whether the snack has collided with something,
        * providing it's active
        */
        false
    }
}

pub fn spawn_snacks(num: usize) -> Vec<Snack> {
        /*
        * TODO: 
        * Generate snacks
        */
        vec![]
}