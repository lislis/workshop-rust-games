use ggez::conf;
use ggez::event;
use ggez::{ContextBuilder, GameResult};

use std::env;
use std::path;

mod game;
use crate::game::{State};
use crate::game::{SCREEN_W, SCREEN_H};

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ref mut ctx, ref mut event_loop) =
        ContextBuilder::new("crab", "lislis & ramonh")
        .window_setup(conf::WindowSetup::default().title("Crab"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_W, SCREEN_H))
        .add_resource_path(resource_dir)
        .build()?;

    let game = &mut State::new(ctx)?;
    event::run(ctx, event_loop, game)
}
