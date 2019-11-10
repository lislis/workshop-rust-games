use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{CLAW_W, CLAW_H, CLAW_S};

pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub struct Claw {
    pub location: Point2,
    body_anchor: Vector2,
    joint_anchor: Vector2,
    w: f32,
    h: f32,
    s: f32
}

impl Claw {
    pub fn new(location: Point2,
               body_anchor: Vector2,
               joint_anchor: Vector2) -> GameResult<Claw> {
        let c = Claw {
            location,
            body_anchor,
            joint_anchor,
            w: CLAW_W,
            h: CLAW_H,
            s: CLAW_S
        };
        Ok(c)
    }

    pub fn update(&mut self, parent_loc: Point2) -> GameResult<&Self> {
        /*
        * TODO: Update claw location according to body's location
        */
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        /*
        * TODO: 
        * 1. Draw a pure red line from the body to the claw
        * 2. Draw the claw image
        */
        Ok(self)
    }

    pub fn get_origin(&self) -> Point2 {
        /*
        * TODO: return calculated origin point
        */
        Point2::new(0., 0.)
    }

    pub fn movedir(&mut self, dir:Directions) -> Vector2 {
        /*
        * TODO: Change joint anchor according to direction
        */
        Vector2::new(0., 0.)
    }
}
