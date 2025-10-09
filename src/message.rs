use ggez::{graphics, Context};

use crate::ggez_utility::*;
use crate::mbtext::*;

pub struct Message {
    pub text: MBText,
    pub duration: f32,
    pub elapsed: f32,
}

impl Message {
    pub fn new(text: String, font: &str, duration: f32, ctx: &mut Context) -> Message {
        Message {
            text: MBText::new(text, font, WHITE, 128.0, ctx),
            duration: duration,
            elapsed: 0.0,
        }
    }
    pub fn update(&mut self, dt: std::time::Duration) {
        self.elapsed += dt.as_millis() as f32;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) {
        let text_pos = self.text.center(ctx);
        self.text.draw(text_pos, ctx, canvas);
    }
}
