use ggez::graphics::{self};
use ggez::Context;

use crate::assets::*;
use crate::ggez_utility::*;

pub struct Message {
    pub text: graphics::Text,
    pub duration: f32,
    pub elapsed: f32,
}

impl Message {
    pub fn new(text: String, duration: f32, assets: &Assets) -> Message {
        Message {
            text: graphics::Text::new((text, assets.title_font, 64.0)),
            duration: duration,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.elapsed += dt.as_millis() as f32;
    }

    pub fn draw(&self, ctx: &mut Context) {
        let text_pos = get_text_center(ctx, &self.text);
        let _ = graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::new()
                .color(graphics::Color::from((255, 255, 255, 255)))
                .dest(text_pos),
        );
    }
}
