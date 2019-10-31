use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra as na;
use ggez::Context;
use rand::*;

use crate::assets::*;
use crate::explosion::*;
use crate::ggez_utility::*;

pub enum TurretState {
    Firing,
    Resting,
    //todo Rotating
}

pub struct Turret {
    pub rotation: f32,
    pub raw_text: String,
    pub text: graphics::Text,
    pub explosions: Vec<Explosion>,
    pub state: TurretState,
    pub src_pixel_width: f32,
    pub src_pixel_height: f32,
}
impl Scalable for Turret {
    fn pct_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.5, 0.9)
    }
    fn pct_dimensions(&self) -> (f32, f32) {
        (0.035, 0.05)
    }
    fn src_pixel_dimensions(&self) -> (f32, f32) {
        (self.src_pixel_width, self.src_pixel_height)
    }
}
impl Turret {
    pub fn new(assets: &Assets) -> Turret {
        let mut rng = rand::thread_rng();
        let mut explosions = Vec::new();
        for _ in 0..20 {
            let r1 = rng.gen_range(-0.05, 0.05);
            let r2 = rng.gen_range(-0.05, 0.05);
            let t = rng.gen_range(0.0, 1000.0);
            explosions.push(Explosion::new(t, na::Point2::new(0.5 + r1, 0.9 + r2)));
        }

        Turret {
            rotation: 0.0,
            raw_text: "".to_string(),
            text: graphics::Text::new(("", assets.main_font, 24.0)),
            explosions: explosions,
            state: TurretState::Resting,
            src_pixel_width: assets.turret.width() as f32,
            src_pixel_height: assets.turret.height() as f32,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context, _dt: std::time::Duration) {}

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets) {
        let param = DrawParam::new()
            .color(white())
            .scale(self.scale(graphics::size(ctx)))
            .offset(na::Point2::new(0.5, 0.5))
            .rotation(self.rotation)
            .dest(self.pixel_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &assets.turret, param);
        let text_param = DrawParam::new()
            .color(white())
            .dest(self.pixel_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &self.text, text_param);
    }

    pub fn draw_lives(&self, lives: usize, ctx: &mut Context, assets: &mut Assets) {
        let scale = self.scale(graphics::size(ctx));

        for i in 0..lives {
            let param = DrawParam::new()
                .color(white())
                .scale(scale * 0.5)
                .offset(na::Point2::new(0.5, 0.5))
                .dest(to_screen_pos(
                    (0.95 + 0.03 * i as f32, 0.925),
                    graphics::size(ctx),
                ));
            let _ = graphics::draw(ctx, &assets.turret, param);
        }
    }
}
