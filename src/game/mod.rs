//use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};

mod config;
mod state;
mod crab;
mod player;
mod claw;
use crate::game::claw::{Directions};
mod assets;
mod snacks;

pub use crate::game::state::{State};
pub use crate::game::config::{SCREEN_W, SCREEN_H};

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for s in self.snacks.iter_mut() {
            s.update()?;
        }
        self.crab.update(self.screen_width)?;
        self.player1.update(self.crab.location)?;
        self.player2.update(self.crab.location)?;
        self.collision_check();
        if !self.assets.bg_sound.playing() {
            let _ = self.assets.bg_sound.play();
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        for s in self.snacks.iter() {
            s.draw(ctx)?;
        }
        self.crab.draw(&self.assets, ctx)?;
        self.player1.draw(ctx, &self.assets.claw_left)?;
        self.player2.draw(ctx, &self.assets.claw_right)?;

        self.render_ui(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        match keycode {
            KeyCode::W => {
                self.player1.movedir(Directions::Up);
            },
            KeyCode::A => {
                self.player1.movedir(Directions::Left);
            },
            KeyCode::S => {
                self.player1.movedir(Directions::Down);
            },
            KeyCode::D => {
                self.player1.movedir(Directions::Right);
            },
            _ => (),
        }
        match keycode {
            KeyCode::I => {
                self.player2.movedir(Directions::Up);
            },
            KeyCode::J => {
                self.player2.movedir(Directions::Left);
            },
            KeyCode::K => {
                self.player2.movedir(Directions::Down);
            },
            KeyCode::L => {
                self.player2.movedir(Directions::Right);
            },
            _ => (),
        }
    }
}
