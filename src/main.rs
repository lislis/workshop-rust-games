use ggez;
//use ggez::audio;
//use ggez::audio::SoundSource;
use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand;
use nalgebra as na;

use std::env;
use std::path;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

const CRAB_H: f32 = 150.0;
const CRAB_W: f32 = 100.0;
const CRAB_S: f32 = 1.5;

const CLAW_W: f32 = 14.0;
const CLAW_H: f32 = 50.0;
const CLAW_S: f32 = 5.0;

enum Directions {
    Up,
    Down,
    Left,
    Right
}

struct Claw {
    location: Point2,
    body_anchor: Vector2,
    joint_anchor: Vector2,
    w: f32,
    h: f32,
    s: f32
}

impl Claw {
    fn new(location: Point2,
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

    fn update(&mut self, parent_loc: Point2) -> GameResult<&Self> {
        self.location = parent_loc;
        Ok(self)
    }

    fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let b_anchor = Point2::new(self.location.x + self.body_anchor.x,
                                   self.location.y + self.body_anchor.y);
        let j_anchor = Point2::new(self.location.x + self.joint_anchor.x,
                                   self.location.y + self.joint_anchor.y);
        let claw_origin = Point2::new(j_anchor.x - self.w, j_anchor.y - self.h);
        let arm = graphics::Mesh::new_line(ctx,
                                           &[b_anchor,
                                             j_anchor],
                                           10.,
                                           graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::draw(ctx, &arm, graphics::DrawParam::default())?;

        let drawparams = graphics::DrawParam::new()
            .dest(claw_origin)
            .rotation(0.0)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, drawparams)?;

        Ok(self)
    }

    fn movedir(&mut self, dir:Directions) -> Vector2 {
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

struct Crab {
    location: Point2,
    velocity: Vector2,
    w: f32,
    h: f32,
    s: f32,
    claw1: Claw,
    claw2: Claw
}

impl Crab {
    fn new(location: Point2) -> GameResult<Crab> {
        let c = Crab {
            location,
            velocity: Vector2::new(CRAB_S, 0.0),
            w: CRAB_W,
            h: CRAB_H,
            s: CRAB_S,
            claw1: Claw::new(location,
                             Vector2::new(CLAW_W, CRAB_H / 2.),
                             Vector2::new(-30., -20.))?, // magical positioning
            claw2: Claw::new(location,
                             Vector2::new(CRAB_W + 30.0, CRAB_H / 2.),
                             Vector2::new(170.0, -20.0))? // magical positioning pt2
        };
        Ok(c)
    }

    fn update(&mut self, max_screen: f32) -> GameResult<&Self> {
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

    fn draw(&self, assets: &Assets, ctx: &mut Context) -> GameResult<&Self> {
        let drawparams = graphics::DrawParam::new()
            .dest(self.location)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, &assets.crab_image, drawparams)?;

        self.claw1.draw(ctx, &assets.claw_left)?;
        self.claw2.draw(ctx, &assets.claw_right)?;
        Ok(self)
    }
}

// tbd snacks

struct Assets {
    crab_image: graphics::Image,
    claw_left: graphics::Image,
    claw_right: graphics::Image,
    font: graphics::Font,
    //bg_sound: audio::Source,
    //snap_sound: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let crab_image = graphics::Image::new(ctx, "/crab.png")?;
        let claw_left =  graphics::Image::new(ctx, "/claw_left.png")?;
        let claw_right =  graphics::Image::new(ctx, "/claw_right.png")?;
        let font =  graphics::Font::new(ctx, "/Airstream.ttf")?;
        //bg_sound =  audio::Source::new(ctx, "")?;
        //snap_sound =  audio::Source::new(ctx, "")?;
        Ok(Assets {
            crab_image,
            claw_left,
            claw_right,
            font,
            //bg_sound,
            //snap_sound
        })
    }
}

enum States {
    Main,
}


struct State {
    dt: std::time::Duration,
    player1_score: usize,
    player2_score: usize,
    state: States,
    crab: Crab,
    screen_width: f32,
    screen_height: f32,
    assets: Assets
}

impl State {
    fn new(ctx: &mut Context) -> ggez::GameResult<State> {
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
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        self.crab.update(self.screen_width)?;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        self.crab.draw(&self.assets, ctx)?;
        self.render_ui(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool) {
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


pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("crab", "lislis & ramonh")
        .window_setup(conf::WindowSetup::default().title("Crab"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir)
        .build()?;

    let game = &mut State::new(ctx)?;
    event::run(ctx, event_loop, game)
}
