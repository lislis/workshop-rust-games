use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::audio;

pub struct Assets {
    pub crab_image: graphics::Image,
    pub claw_left: graphics::Image,
    pub claw_right: graphics::Image,
    pub bg_image: graphics::Image,
    pub snack_image: graphics::Image,
    pub font: graphics::Font,
    pub bg_sound: audio::Source,
    pub snap_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let crab_image = graphics::Image::new(ctx, "/crab.png")?;
        let claw_left =  graphics::Image::new(ctx, "/claw_left.png")?;
        let claw_right =  graphics::Image::new(ctx, "/claw_right.png")?;
        let bg_image =  graphics::Image::new(ctx, "/sand.png")?;
        let snack_image =  graphics::Image::new(ctx, "/snack.png")?;
        let font =  graphics::Font::new(ctx, "/Airstream.ttf")?;
        let bg_sound =  audio::Source::new(ctx, "/Modern-Jazz-Samba-CUT.mp3")?;
        let snap_sound =  audio::Source::new(ctx, "/woopwoop.mp3")?;
        Ok(Assets {
            crab_image,
            claw_left,
            claw_right,
            bg_image,
            snack_image,
            font,
            bg_sound,
            snap_sound
        })
    }
}
