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
        self.location += self.velocity;
        if self.location.y > SCREEN_H {
            self.active = false;
        }
        if !self.active {
            self.location = Point2::new(rand::random::<f32>() * SCREEN_W,
                                        -SNACK_W);
            self.velocity = Vector2::new(0.0,
                                         rand::random::<f32>() * 2.0 + 0.1);
            self.active = true;
        }
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<&Self> {
        if self.active {
            let rect = graphics::Rect::new(self.location.x,
                                           self.location.y,
                                           self.w,
                                           self.w);
            let r = graphics::Mesh::new_rectangle(ctx,
                                                  graphics::DrawMode::fill(),
                                                  rect,
                                                  graphics::BLACK)?;
            graphics::draw(ctx, &r, graphics::DrawParam::default())?;
        }
        Ok(self)
    }

    pub fn collides_with(&mut self, other: Point2) -> bool {
        if self.active {
            let distance = self.location - other;
            if distance.norm() < self.w {
                self.active = false;
                return true
            }
        }
        false
    }
}

pub fn spawn_snacks(num: usize) -> Vec<Snack> {
    (0..num).map(|_v| Snack::new()
                 .expect("Could not create snack")).collect()
}
