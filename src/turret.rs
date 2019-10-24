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
}
impl Scalable for Turret {
    fn get_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.5, 0.9)
    }
    fn get_dimensions(&self) -> (f32, f32) {
        (0.05, 0.05)
    }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32, f32) {
        (assets.turret.width() as f32, assets.turret.height() as f32)
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
        }
    }

    pub fn update(&mut self, _ctx: &mut Context, _dt: std::time::Duration) {
    } 

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets) {
        let param = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .scale(self.get_texture_scale(graphics::size(ctx), assets))
            .offset(na::Point2::new(0.5, 0.5))
            .rotation(self.rotation)
            .dest(self.get_screen_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &assets.turret, param);
        let text_param = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .dest(self.get_screen_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &self.text, text_param);
    }

    pub fn draw_lives(&self, lives: usize, ctx: &mut Context, assets: &mut Assets) {
        let scale = self.get_texture_scale(graphics::size(ctx), assets);

        for i in 0..lives {
            let param = DrawParam::new()
                .color(Color::from((255, 255, 255, 255)))
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
