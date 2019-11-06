use crate::assets::*;
use crate::ggez_utility::*;
use ggez::graphics::{self, Color};
use ggez::nalgebra as na;
use ggez::Context;

pub struct Crosshair {
    pub elapsed: u32,
    pub src_pixel_width: f32,
    pub src_pixel_height: f32,
}

const CROSSHAIR_TIME: u32 = 1000;

impl Crosshair {
    pub fn update(&mut self, dt: std::time::Duration) {
        self.elapsed = (self.elapsed + dt.as_millis() as u32) % CROSSHAIR_TIME;
    }

    pub fn draw(&mut self, pos: na::Point2<f32>, ctx: &mut Context, assets: &Assets) {
        let pct = self.elapsed as f32 / CROSSHAIR_TIME as f32;
        let mut color = (510.0 * pct) as u32;
        if color > 255 {
            color = 255 - (color - 255);
        }

        let crosshair_params = graphics::DrawParam::new()
            .color(Color::from((255, color as u8, color as u8, 255)))
            .dest(pos)
            .scale(self.scale(graphics::size(ctx)))
            .offset(na::Point2::new(0.5, 0.5));
        let _ = graphics::draw(ctx, &assets.crosshair, crosshair_params);
    }
}
impl Scalable for Crosshair {
    fn pct_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.0, 0.0)
    }
    fn pct_dimensions(&self) -> (f32, f32) {
        (0.090, 0.125)
    }
    fn src_pixel_dimensions(&self) -> (f32, f32) {
        (self.src_pixel_width, self.src_pixel_height)
    }
}
