use ggez;
use ggez::audio;
use ggez::audio::SoundSource;
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

/// Create a unit vector representing the
/// given angle (in radians)
fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

/// Makes a random `Vector2` with the given max magnitude.
fn random_vec(max_magnitude: f32) -> Vector2 {
    let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    let mag = rand::random::<f32>() * max_magnitude;
    vec_from_angle(angle) * (mag)
}


/// Translates the world coordinate system, which
/// has Y pointing up and the origin at the center,
/// to the screen coordinate system, which has Y
/// pointing downward and the origin at the top-left,
fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Point2) -> Point2 {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2::new(x, y)
}


struct Body {
    location: Point2,
    velocity: Vector2,
    heading: f32,
    avelocity: f32
}

impl Body  {
    fn new(x:f32, y:f32) -> Body {
        let b = Body {
            location: Point2::new(x, y),
            velocity: na::zero(),
            heading: 0.,
            avelocity: 0.
        };
        b
    }
}

struct Claw {
    body: Body
}

impl Claw {
    fn new(x:f32, y:f32) -> Claw {
        let c = Claw {
            body: Body::new(x, y)
        };
        c
    }
}

struct Crab {
    body: Body,
    claw1: Claw,
    claw2: Claw
}

impl Crab {
    fn new(x:f32, y:f32) -> Crab {
        let c = Crab {
            body: Body::new(x, y),
            claw1: Claw::new(0.0, 0.0),
            claw2: Claw::new(0.0, 0.0 )
        };
        c
    }
}

// tbd snacks

struct Assets {
    crab_image: graphics::Image,
    claw_left: graphics::Image,
    claw_right: graphics::Image,
    //font: graphics::Font,
    //bg_sound: audio::Source,
    //snap_sound: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let crab_image = graphics::Image::new(ctx, "/crab.png")?;
        let claw_left =  graphics::Image::new(ctx, "/claw_left.png")?;
        let claw_right =  graphics::Image::new(ctx, "/claw_right.png")?;
        //font =  graphics::Font::new(ctx, "")?;
        //bg_sound =  audio::Source::new(ctx, "")?;
        //snap_sound =  audio::Source::new(ctx, "")?;
        Ok(Assets {
            crab_image,
            claw_left,
            claw_right,
            //font,
            //bg_sound,
            //snap_sound
        })
    }
}



struct State {
    dt: std::time::Duration,
    player1_score: usize,
    player2_score: usize,
    state: String,
    crab: Crab,
    screen_width: f32,
    screen_height: f32
}

impl State {
    fn new(ctx: &mut Context) -> ggez::GameResult<State> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;
        let (width, height) = ggez::graphics::drawable_size(ctx);

        let s = State {
            dt: std::time::Duration::new(0, 0),
            player1_score: 0,
            player2_score: 0,
            state: String::from("play"),
            crab: Crab::new(0.0, 0.0),
            screen_width: width,
            screen_height: height,
        };
        Ok(s)
    }

}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool) {
        match keycode {
            KeyCode::W => {
                println!("UP")
            },
            _ => (),
        }
    }
}


pub fn main() -> ggez::GameResult {
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
