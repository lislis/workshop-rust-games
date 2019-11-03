use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::config::{CLAW_W, CLAW_H, CLAW_S};

pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub struct Claw {
    pub location: Point2,
    body_anchor: Vector2,
    pub joint_anchor: Vector2,
    pub w: f32,
    pub h: f32,
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
        self.location = parent_loc;
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let b_anchor = self.location + self.body_anchor;
        let j_anchor = self.location + self.joint_anchor;

        let arm = graphics::Mesh::new_line(ctx,
                                           &[b_anchor,
                                             j_anchor],
                                           10.,
                                           graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::draw(ctx, &arm, graphics::DrawParam::default())?;

        let drawparams = graphics::DrawParam::new()
            .dest(self.get_origin())
            .rotation(0.0)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, drawparams)?;

        Ok(self)
    }

    pub fn get_origin(&self) -> Point2 {
        let j_anchor = self.location + self.joint_anchor;
        let claw_origin = Point2::new(j_anchor.x - self.w / 2., j_anchor.y - self.h);
        claw_origin
    }

    pub fn movedir(&mut self, dir:Directions) -> Vector2 {
        match dir {
            Directions::Up => {
                self.joint_anchor.y -= self.s;
            },
            Directions::Down => {
                self.joint_anchor.y += self.s;
            },
            Directions::Right => {
                self.joint_anchor.x += self.s;
            },
            Directions::Left => {
                self.joint_anchor.x -= self.s;
            }
        }
        self.joint_anchor
    }
}
