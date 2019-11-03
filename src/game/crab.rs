use ggez::{Context, GameResult};
use ggez::graphics;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::assets::Assets;
use crate::game::claw::{Claw};

use crate::config::{CRAB_W,
                    CRAB_H,
                    CRAB_S,
                    CLAW_W};


pub struct Crab {
    location: Point2,
    velocity: Vector2,
    w: f32,
    h: f32,
    s: f32,
    pub claw1: Claw,
    pub claw2: Claw
}

impl Crab {
    pub fn new(location: Point2) -> GameResult<Crab> {
        let c = Crab {
            location,
            velocity: Vector2::new(CRAB_S, 0.0),
            w: CRAB_W,
            h: CRAB_H,
            s: CRAB_S,
            claw1: Claw::new(location,
                             Vector2::new(CLAW_W - 20., CRAB_H / 2.),
                             Vector2::new(-30., -20.))?, // magical positioning
            claw2: Claw::new(location,
                             Vector2::new(CRAB_W + 30.0, CRAB_H / 2.),
                             Vector2::new(170.0, -20.0))? // magical positioning pt2
        };
        Ok(c)
    }

    pub fn update(&mut self, max_screen: f32) -> GameResult<&Self> {
        self.location.x += self.velocity.x;

        if self.location.x + (self.w * 2.) >= max_screen {
            self.velocity.x = - self.s;
        } else if self.location.x < self.w {
            self.velocity.x = self.s;
        }

        self.claw1.update(self.location)?;
        self.claw2.update(self.location)?;
        Ok(self)
    }

    pub fn draw(&self, assets: &Assets, ctx: &mut Context) -> GameResult<&Self> {
        let drawparams = graphics::DrawParam::new()
            .dest(self.location)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, &assets.crab_image, drawparams)?;

        self.claw1.draw(ctx, &assets.claw_left)?;
        self.claw2.draw(ctx, &assets.claw_right)?;
        Ok(self)
    }
}
