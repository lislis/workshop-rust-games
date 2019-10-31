use ggez::{Context, GameResult};
use ggez::graphics;
use rand;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::config::{SNACK_W};

pub struct Snack {
    location: Point2,
    velocity: Vector2,
    w: f32
}

impl Snack {
    fn new (location: Point2) -> GameResult<Snack> {
        let s = Snack {
            location,
            velocity: Vector2::new(0.0, 1.0),
            w: SNACK_W
        };
        Ok(s)
    }

    pub fn update(&mut self) -> GameResult<&Self> {
        self.location += self.velocity;
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<&Self> {
        let rect = graphics::Rect::new(self.location.x,
                                       self.location.y,
                                       self.w,
                                       self.w);
        let r = graphics::Mesh::new_rectangle(ctx,
                                              graphics::DrawMode::fill(),
                                              rect,
                                              graphics::BLACK)?;
        graphics::draw(ctx, &r, graphics::DrawParam::default())?;
        Ok(self)
    }
}

pub fn spawn_snacks(num: usize) -> Vec<Snack> {
    (0..num).map(|v| Snack::new(Point2::new(v as f32 * 20., 0.))
                 .expect("Could not create snack")).collect()
}
