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
    pub number_font: graphics::Font,
    pub turret: graphics::Image,
    pub background: graphics::Image,
    pub stars1: graphics::Image,
    pub stars2: graphics::Image,
    pub explosion: graphics::Image,
    pub explosion_sound: audio::Source,
    pub clap_sound: audio::Source,
    pub launch_sound: audio::Source,
    pub fail_sound: audio::Source,
    pub laser_sound: audio::Source,
    pub music: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context, start_bg: String) -> Assets {
        Assets {
            add_ship: graphics::Image::new(ctx, "/add-ship.png").unwrap(),
            sub_ship: graphics::Image::new(ctx, "/sub-ship.png").unwrap(),
            mul_ship: graphics::Image::new(ctx, "/mul-ship.png").unwrap(),
            crosshair: graphics::Image::new(ctx, "/crosshair.png").unwrap(),
            div_ship: graphics::Image::new(ctx, "/div-ship.png").unwrap(),
            title_font: graphics::Font::new(ctx, "/title.ttf").unwrap(),
            main_font: graphics::Font::new(ctx, "/main.ttf").unwrap(),
            number_font: graphics::Font::new(ctx, "/number.ttf").unwrap(),
            turret: graphics::Image::new(ctx, "/turret.png").unwrap(),
            background: graphics::Image::new(ctx, start_bg).unwrap(),
            stars1: graphics::Image::new(ctx, "/stars1.png").unwrap(),
            stars2: graphics::Image::new(ctx, "/stars2.png").unwrap(),
            explosion: graphics::Image::new(ctx, "/explosion.png").unwrap(),
            explosion_sound: audio::Source::new(ctx, "/explosion.wav").unwrap(),
            clap_sound: audio::Source::new(ctx, "/clap.ogg").unwrap(),
            launch_sound: audio::Source::new(ctx, "/launch.wav").unwrap(),
            fail_sound: audio::Source::new(ctx, "/fail.ogg").unwrap(),
            laser_sound: audio::Source::new(ctx, "/laser.ogg").unwrap(),
            music: audio::Source::new(ctx, "/music.mp3").unwrap(),
        }
    }
}
