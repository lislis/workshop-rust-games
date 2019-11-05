use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::audio::SoundSource;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{NUM_SNACKS, CRAB_W, CRAB_H, CLAW_W};
use crate::game::assets::Assets;
use crate::game::crab::{Crab};
use crate::game::snacks::{Snack, spawn_snacks};
use crate::game::player::{Player};

pub struct State {
    pub player1: Player,
    pub player2: Player,
    pub crab: Crab,
    pub snacks: Vec<Snack>,
    pub screen_width: f32,
    pub assets: Assets
}

impl State {
    pub fn new(ctx: &mut Context) -> ggez::GameResult<State> {
        println!("Play Crab!");
        println!("Player 1, use WASD!");
        println!("Player 2, use IJKL!");
        println!("Have fun!");

        let assets = Assets::new(ctx)?;
        let (width, height) = ggez::graphics::drawable_size(ctx);
        let crab_origin = Point2::new(width / 2.0 - (CRAB_W / 2.0),
                                      height - CRAB_H);

        let s = State {
            player1: Player::new(crab_origin,
                                 Vector2::new(CLAW_W - 20., CRAB_H / 2.),
                                 Vector2::new(-30., -20.))?,
            player2: Player::new(crab_origin,
                                 Vector2::new(CRAB_W + 30.0, CRAB_H / 2.),
                                 Vector2::new(170.0, -20.0))?,
            crab: Crab::new(crab_origin)?,
            snacks: spawn_snacks(NUM_SNACKS),
            screen_width: width,
            assets: assets
        };
        Ok(s)
    }

    pub fn render_ui(&self, ctx: &mut Context) -> GameResult<&Self> {
        let score_1 = graphics::Text::new((format!("Player 1: #{}",
                                                   self.player1.score),
                                           self.assets.font, 38.0));
        let score_2 = graphics::Text::new((format!("Player 2: #{}",
                                                   self.player2.score),
                                           self.assets.font, 38.0));
        /*
        * TODO:
        * Display the scores on the screen!
        */
        Ok(self)
    }

    pub fn collision_check(&mut self) {
        let c1 = self.player1.claw.get_origin();
        let c2 = self.player2.claw.get_origin();
        /*
        * TODO:
        * Loop over the snacks and check whether they've collided with either claw!
        * If it collides with either:
        * 1. Play the sound
        * 2. Increase player's score
        */
    }
}
