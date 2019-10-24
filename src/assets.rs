use ggez::audio;
use ggez::graphics::{self};
use ggez::Context;

pub struct Assets {
    pub add_ship: graphics::Image,
    pub sub_ship: graphics::Image,
    pub mul_ship: graphics::Image,
    pub div_ship: graphics::Image,
    pub crosshair: graphics::Image,
    pub title_font: graphics::Font,
    pub main_font: graphics::Font,
    pub turret: graphics::Image,
    pub background: graphics::Image,
    pub explosion: graphics::Image,
    pub explosion_sound: audio::Source,
    pub music: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Assets {
        Assets {
            add_ship: graphics::Image::new(ctx, "/add-ship.png").unwrap(),
            sub_ship: graphics::Image::new(ctx, "/sub-ship.png").unwrap(),
            mul_ship: graphics::Image::new(ctx, "/mul-ship.png").unwrap(),
            crosshair: graphics::Image::new(ctx, "/crosshair.png").unwrap(),
            div_ship: graphics::Image::new(ctx, "/div-ship.png").unwrap(),
            title_font: graphics::Font::new(ctx, "/title.ttf").unwrap(),
            main_font: graphics::Font::new(ctx, "/main.ttf").unwrap(),
            turret: graphics::Image::new(ctx, "/turret.png").unwrap(),
            background: graphics::Image::new(ctx, "/spacebg1.jpg").unwrap(),
            explosion: graphics::Image::new(ctx, "/explosion.png").unwrap(),
            explosion_sound: audio::Source::new(ctx, "/explosion.wav").unwrap(),
            music: audio::Source::new(ctx, "/music.ogg").unwrap(),
        }
    }
}
