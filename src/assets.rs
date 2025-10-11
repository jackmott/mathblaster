use ggez::{audio, graphics, Context};

pub struct Assets {
    pub add_ship: graphics::Image,
    pub sub_ship: graphics::Image,
    pub mul_ship: graphics::Image,
    pub div_ship: graphics::Image,
    pub crosshair: graphics::Image,
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
            add_ship: graphics::Image::from_path(ctx, "/add-ship.png").unwrap(),
            sub_ship: graphics::Image::from_path(ctx, "/sub-ship.png").unwrap(),
            mul_ship: graphics::Image::from_path(ctx, "/mul-ship.png").unwrap(),
            crosshair: graphics::Image::from_path(ctx, "/crosshair.png").unwrap(),
            div_ship: graphics::Image::from_path(ctx, "/div-ship.png").unwrap(),
            turret: graphics::Image::from_path(ctx, "/turret.png").unwrap(),
            background: graphics::Image::from_path(ctx, start_bg).unwrap(),
            stars1: graphics::Image::from_path(ctx, "/stars1.png").unwrap(),
            stars2: graphics::Image::from_path(ctx, "/stars2.png").unwrap(),
            explosion: graphics::Image::from_path(ctx, "/explosion.png").unwrap(),
            explosion_sound: audio::Source::new(ctx, "/explosion.wav").expect("Sound"),
            clap_sound: audio::Source::new(ctx, "/clap.ogg").expect("Sound"),
            launch_sound: audio::Source::new(ctx, "/launch.wav").expect("Sound"),
            fail_sound: audio::Source::new(ctx, "/fail.ogg").expect("Sound"),
            laser_sound: audio::Source::new(ctx, "/laser.ogg").expect("Sound"),
            music: audio::Source::new(ctx, "/music.mp3").expect("Sound"),
        }
    }
}

pub struct FontAssets {
    pub title_font: graphics::FontData,
    pub main_font: graphics::FontData,
    pub number_font: graphics::FontData,
}

impl FontAssets {
    pub fn new(ctx: &mut Context) -> FontAssets {
        FontAssets {
            title_font: graphics::FontData::from_path(ctx, "/title.ttf").unwrap(),
            main_font: graphics::FontData::from_path(ctx, "/main.ttf").unwrap(),
            number_font: graphics::FontData::from_path(ctx, "/number.ttf").unwrap(),
        }
    }
}
