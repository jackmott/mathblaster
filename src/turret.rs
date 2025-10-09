extern crate nalgebra as na;

use ggez::{graphics, Context};
use rand::Rng;

use crate::assets::*;
use crate::explosion::*;
use crate::ggez_utility::*;
use crate::mbtext::*;

pub enum TurretState {
    Firing,
    Resting,
    //todo Rotating?
}

pub struct Turret {
    pub rotation: f32,
    pub raw_text: String,
    pub text: MBText,
    pub explosions: Vec<Explosion>,
    pub state: TurretState,
    pub src_pixel_width: f32,
    pub src_pixel_height: f32,
    pub pos: na::Point2<f32>,
}

impl Scalable for Turret {
    fn pct_pos(&self) -> na::Point2<f32> {
        self.pos
    }
    fn pct_dimensions(&self) -> (f32, f32) {
        (0.030, 0.05)
    }
    fn src_pixel_dimensions(&self) -> (f32, f32) {
        (self.src_pixel_width, self.src_pixel_height)
    }
}
impl Turret {
    pub fn new(assets: &Assets, font: &str, ctx: &mut Context) -> Turret {
        let mut rng = rand::rng();
        let mut explosions = Vec::new();
        for _ in 0..20 {
            let r1 = rng.random_range(-0.05..0.05);
            let r2 = rng.random_range(-0.05..0.05);
            let t = rng.random_range(0.0..1000.0);
            explosions.push(Explosion::new(t, na::Point2::new(0.5 + r1, 0.9 + r2)));
        }

        Turret {
            rotation: 0.0,
            raw_text: "".to_string(),
            text: MBText::new("".to_string(), font, WHITE, 24.0, ctx),
            explosions: explosions,
            state: TurretState::Resting,
            src_pixel_width: assets.turret.width() as f32,
            src_pixel_height: assets.turret.height() as f32,
            pos: na::Point2::new(0.5, 0.9),
        }
    }

    pub fn update(&mut self, _ctx: &mut Context, _dt: std::time::Duration) {}

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets, canvas: &mut graphics::Canvas) {
        let param = graphics::DrawParam::new()
            .color(WHITE)
            .scale(self.scale(ctx.gfx.size()))
            .offset(na::Point2::new(0.5, 0.5))
            .rotation(self.rotation)
            .dest(self.pixel_pos(ctx.gfx.size()));
        canvas.draw(&assets.turret, param);
        self.text
            .draw_horizontal_center(ctx.gfx.size().1 * 0.9, ctx, canvas);
    }

    pub fn draw_lives(
        &self,
        lives: usize,
        ctx: &mut Context,
        assets: &mut Assets,
        canvas: &mut graphics::Canvas,
    ) {
        let scale = self.scale(ctx.gfx.size());

        for i in 0..lives {
            let param = graphics::DrawParam::new()
                .color(WHITE)
                .scale(scale * 0.5)
                .offset(na::Point2::new(0.5, 0.5))
                .dest(to_screen_pos(
                    (0.95 + 0.03 * i as f32, 0.925),
                    ctx.gfx.size(),
                ));
            canvas.draw(&assets.turret, param);
        }
    }
}
