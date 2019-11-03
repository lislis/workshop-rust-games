//use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use rand;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

mod claw;
use crate::game::claw::{Directions};

mod assets;
use crate::game::assets::Assets;

mod crab;
use crate::game::crab::{Crab};

mod snacks;
use crate::game::snacks::{Snack, spawn_snacks};

use crate::config::{CRAB_W, CRAB_H, NUM_SNACKS};

enum States {
    Main,
}

pub struct State {
    dt: std::time::Duration,
    player1_score: usize,
    player2_score: usize,
    state: States,
    crab: Crab,
    snacks: Vec<Snack>,
    screen_width: f32,
    screen_height: f32,
    assets: Assets
}

impl State {
    pub fn new(ctx: &mut Context) -> ggez::GameResult<State> {
        println!("Play Crab!");
        println!("Player 1, use WASD!");
        println!("Player 2, use IJKL!");
        println!("There will be a pause button eventually!");

        let assets = Assets::new(ctx)?;
        let (width, height) = ggez::graphics::drawable_size(ctx);

        let s = State {
            dt: std::time::Duration::new(0, 0),
            player1_score: 0,
            player2_score: 0,
            state: States::Main,
            crab: Crab::new(Point2::new(width / 2.0 - (CRAB_W / 2.0),
                                        height - CRAB_H))?,
            snacks: spawn_snacks(NUM_SNACKS),
            screen_width: width,
            screen_height: height,
            assets: assets
        };
        Ok(s)
    }

    fn render_ui(&self, ctx: &mut Context) -> GameResult<&Self> {
        let score_1 = graphics::Text::new((format!("Player 1: #{}", self.player1_score),
                                           self.assets.font, 38.0));
        let score_2 = graphics::Text::new((format!("Player 2: #{}", self.player2_score),
                                           self.assets.font, 38.0));
        graphics::draw(ctx, &score_1, (Point2::new(10.0, 10.0),
                                       0.0,
                                       graphics::BLACK))?;
        graphics::draw(ctx, &score_2, (Point2::new(self.screen_width - 180.00, 10.0),
                                       0.0,
                                       graphics::BLACK))?;

        Ok(self)
    }

    fn collision_check(&mut self) {
        let c1 = self.crab.claw1.get_origin();
        let c2 = self.crab.claw2.get_origin();

        for s in self.snacks.iter_mut() {
            if s.collides_with(c1) {
                self.player1_score += 1;
            }
            if s.collides_with(c2) {
                self.player2_score += 1;
            }
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        self.crab.update(self.screen_width)?;
        for s in self.snacks.iter_mut() {
            s.update()?;
        }
        //if !self.assets.bg_sound.playing() {
        //    let _ = self.assets.bg_sound.play();
        //}
        self.collision_check();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        for s in self.snacks.iter() {
            s.draw(ctx)?;
        }
        self.crab.draw(&self.assets, ctx)?;
        self.render_ui(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
    //    _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => {
                self.crab.claw1.movedir(Directions::Up);
            },
            KeyCode::A => {
                self.crab.claw1.movedir(Directions::Left);
            },
            KeyCode::S => {
                self.crab.claw1.movedir(Directions::Down);
            },
            KeyCode::D => {
                self.crab.claw1.movedir(Directions::Right);
            },
            _ => (),
        }
        match keycode {
            KeyCode::I => {
                self.crab.claw2.movedir(Directions::Up);
            },
            KeyCode::J => {
                self.crab.claw2.movedir(Directions::Left);
            },
            KeyCode::K => {
                self.crab.claw2.movedir(Directions::Down);
            },
            KeyCode::L => {
                self.crab.claw2.movedir(Directions::Right);
            },
            _ => (),
        }
    }
}
