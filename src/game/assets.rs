use ggez::{Context, GameResult};
use ggez::graphics;

pub struct Assets {
    pub crab_image: graphics::Image,
    pub claw_left: graphics::Image,
    pub claw_right: graphics::Image,
    pub font: graphics::Font,
    //bg_sound: audio::Source,
    //snap_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
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
