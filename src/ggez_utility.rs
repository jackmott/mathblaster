use ggez::graphics::{self,Color};
use ggez::nalgebra as na;
use ggez::Context;

use crate::assets::*;

pub fn white() -> graphics::Color {
    graphics::Color::from((255,255,255,255))
}
pub fn blue() -> graphics::Color {
    graphics::Color::from((0,0,255,255))
}
pub fn gray() -> graphics::Color {
    graphics::Color::from((128,128,128,255))
}

pub trait Scalable {
    fn pct_dimensions(&self) -> (f32, f32);
    fn src_pixel_dimensions(&self) -> (f32, f32);
    fn pct_pos(&self) -> na::Point2<f32>;
    fn dest_pixel_dimensions(&self, screen_dimensions: (f32, f32)) -> (f32, f32) {
        let (w, h) = self.pct_dimensions();
        (w * screen_dimensions.0, h * screen_dimensions.1)
    }    
    fn scale(&self, window_dimensions: (f32, f32)) -> na::Vector2<f32> {
        let (sw, sh) = self.dest_pixel_dimensions(window_dimensions);
        let (tw, th) = self.src_pixel_dimensions();
        // only use screen width for scaling
        na::Vector2::new(sw / tw, sh / th)
    }
    fn pixel_pos(&self, screen_dimensions: (f32, f32)) -> na::Point2<f32> {
        let p = self.pct_pos();
        na::Point2::new(p[0] * screen_dimensions.0, p[1] * screen_dimensions.1)
    }
}

pub fn to_screen_pos(pos: (f32, f32), screen_dimensions: (f32, f32)) -> na::Point2<f32> {
    na::Point2::new(pos.0 * screen_dimensions.0, pos.1 * screen_dimensions.1)
}
/*
pub fn get_text_center(ctx: &mut Context, text: &graphics::Text) -> na::Point2<f32> {
    let window_dim = graphics::size(ctx);
    let text_dim = text.dimensions(ctx);
    na::Point2::new(
        window_dim.0 / 2.0 - text_dim.0 as f32 / 2.0,
        window_dim.1 / 2.0 - text_dim.1 as f32 / 2.0,
    )
}*/

pub fn lerp_color(a:Color, b:Color, pct: f32) -> Color {
    let red = a.r * (1.0 - pct) + b.r * pct;
    let green = a.g * (1.0 - pct) + b.g * pct;
    let blue = a.b * (1.0 - pct) + b.b * pct;
    let alpha = a.a * (1.0 - pct) + b.a * pct;
    Color::new(red,green,blue,alpha)
}
