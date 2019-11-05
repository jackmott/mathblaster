use crate::ggez_utility::*;
use ggez::graphics::{self, Color};
use ggez::nalgebra as na;
use ggez::Context;

const TEXT_TIME: u32 = 1000;

pub struct MBText {
    pub text: graphics::Text,
    pub actual_w: f32,
    pub actual_h: f32,
    pub w: f32,
    pub h: f32,
    pub color1: Color,
    pub color2: Color,
    pub elapsed: u32,
}

impl MBText {
    pub fn new(
        text: String,
        font: &graphics::Font,
        color: Color,
        size: f32,
        context: &mut Context,
    ) -> MBText {
        MBText::new_blink(text, font, color, color, size, context)
    }

    pub fn new_blink(
        text: String,
        font: &graphics::Font,
        color1: Color,
        color2: Color,
        size: f32,
        context: &mut Context,
    ) -> MBText {
        let text = graphics::Text::new((text, *font, size));
        let dim = text.dimensions(context);
        MBText {
            text: text,
            actual_w: dim.0 as f32,
            actual_h: dim.1 as f32,
            color1: color1,
            color2: color2,
            w: dim.0 as f32 / 1920.0,
            h: dim.1 as f32 / 1080.0,
            elapsed: 0,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.elapsed = (self.elapsed + dt.as_millis() as u32) % TEXT_TIME;
    }

    pub fn draw(&self, pos: na::Point2<f32>, ctx: &mut Context) {
        let mut pct = (self.elapsed as f32 / TEXT_TIME as f32) * 2.0;
        if pct > 1.0 {
            pct = 1.0 - (pct - 1.0);
        }
        let color = lerp_color(self.color1, self.color2, pct);
        let _ = graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::new()
                .color(color)
                .dest(pos)
                .scale(self.scale(graphics::size(ctx))),
        );
    }

    pub fn draw_color(&self, pos: na::Point2<f32>, color: Color, ctx: &mut Context) {
        let _ = graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::new()
                .color(color)
                .dest(pos)
                .scale(self.scale(graphics::size(ctx))),
        );
    }

    pub fn draw_center(&self, ctx: &mut Context) {
        let pos = self.center(ctx);
        self.draw(pos, ctx);
    }

    pub fn draw_horizontal_center(&self, y: f32, ctx: &mut Context) {
        let center = self.center(ctx);
        self.draw(na::Point2::new(center[0], y), ctx);
    }

    pub fn center(&self, ctx: &mut Context) -> na::Point2<f32> {
        let window_dim = graphics::size(ctx);
        let text_dim = self.dest_pixel_dimensions(window_dim);
        na::Point2::new(
            window_dim.0 / 2.0 - text_dim.0 as f32 / 2.0,
            window_dim.1 / 2.0 - text_dim.1 as f32 / 2.0,
        )
    }
}

impl Scalable for MBText {
    fn pct_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.0, 0.0)
    }

    fn pct_dimensions(&self) -> (f32, f32) {
        (self.w, self.h)
    }

    fn src_pixel_dimensions(&self) -> (f32, f32) {
        (self.actual_w, self.actual_h)
    }
}
