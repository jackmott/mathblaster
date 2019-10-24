use ggez::graphics::{self};
use ggez::nalgebra as na;
use ggez::Context;

use crate::assets::*;

pub trait Scalable {
    fn get_dimensions(&self) -> (f32, f32);
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32, f32);
    fn get_pos(&self) -> na::Point2<f32>;
    fn get_screen_dimensions(&self, screen_dimensions: (f32, f32)) -> (f32, f32) {
        let (w, h) = self.get_dimensions();
        (w * screen_dimensions.0, h * screen_dimensions.1)
    }
    fn get_texture_scale(
        &self,
        screen_dimensions: (f32, f32),
        assets: &Assets,
    ) -> na::Vector2<f32> {
        let (sw, _) = self.get_screen_dimensions(screen_dimensions);
        let (tw, th) = self.get_texture_dimensions(assets);
        // only use screen width for scaling
        na::Vector2::new(sw / tw, sw / th)
    }
    fn get_screen_pos(&self, screen_dimensions: (f32, f32)) -> na::Point2<f32> {
        let p = self.get_pos();
        na::Point2::new(p[0] * screen_dimensions.0, p[1] * screen_dimensions.1)
    }
}

pub fn to_screen_pos(pos: (f32, f32), screen_dimensions: (f32, f32)) -> na::Point2<f32> {
    na::Point2::new(pos.0 * screen_dimensions.0, pos.1 * screen_dimensions.1)
}

pub fn get_text_center(ctx: &mut Context, text: &graphics::Text) -> na::Point2<f32> {
    let window_dim = graphics::size(ctx);
    let text_dim = text.dimensions(ctx);
    na::Point2::new(
        window_dim.0 / 2.0 - text_dim.0 as f32 / 2.0,
        window_dim.1 / 2.0 - text_dim.1 as f32 / 2.0,
    )
}

pub fn lerp(a:f32,b:f32,pct:f32) -> f32 {
    a * (1.0-pct) + b*pct
}
