use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra as na;

use crate::assets::*;
use crate::explosion::*;
use crate::ggez_utility::*;
use crate::level::*;
use crate::turret::*;
use ggez::Context;

#[derive(PartialEq)]
pub enum AlienState {
    Alive,
    Exploding,
    Dead,
}
pub struct Alien {
    pub operation: Operation,
    pub speed: f32,
    pub pos: na::Point2<f32>,
    pub text: graphics::Text,
    pub answer: i32,
    pub explosion: Explosion,
    pub state: AlienState,
    pub src_pixel_width: f32,
    pub src_pixel_height: f32,
}
impl Scalable for Alien {
    fn pct_pos(&self) -> na::Point2<f32> {
        self.pos
    }
    fn pct_dimensions(&self) -> (f32, f32) {
        (0.045, 0.07)
    }
    fn src_pixel_dimensions(&self) -> (f32, f32) {
        (self.src_pixel_width, self.src_pixel_height)
    }
}
impl Alien {
    pub fn update(&mut self, turret: &mut Turret, ctx: &mut Context, dt: std::time::Duration) {
        if self.state != AlienState::Dead {
            let sec = dt.as_millis() as f32 / 100000.0;
            if self.pos[1] < 0.07 {
                self.pos = self.pos + na::Vector2::new(0.0, self.speed * 3. * sec);
            } else {
                self.pos = self.pos + na::Vector2::new(0.0, self.speed * sec);
            }
            if self.state == AlienState::Exploding {
                self.explosion.update(ctx, dt);
            }
            if self.explosion.elapsed > self.explosion.duration {
                self.state = AlienState::Dead;
                turret.state = TurretState::Resting;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
        if self.state != AlienState::Dead {
            if self.explosion.elapsed < self.explosion.duration / 2.0 {
                let params = DrawParam::new()
                    .color(Color::from((255, 255, 255, 255)))
                    .dest(self.pixel_pos(graphics::size(ctx)))
                    .scale(self.scale(graphics::size(ctx)))
                    .offset(na::Point2::new(0.5, 0.5));
                let img = match self.operation {
                    Operation::Add => &assets.add_ship,
                    Operation::Subtract => &assets.sub_ship,
                    Operation::Multiply => &assets.mul_ship,
                    Operation::Divide => &assets.div_ship,
                };
                let _ = graphics::draw(ctx, img, params);

                let tw = self.text.width(ctx) as f32;
                let (sw, sh) = self.dest_pixel_dimensions(graphics::size(ctx));
                let offsetx = -sw / 2.0 + (sw - tw) / 2.0;
                let offsety = -sh / 1.2;

                let offset = na::Vector2::new(offsetx, offsety);

                let text_param = DrawParam::new()
                    .color(Color::from((255, 255, 255, 255)))
                    .dest(self.pixel_pos(graphics::size(ctx)) + offset);
                let _ = graphics::draw(ctx, &self.text, text_param);
            }
        }

        if self.state == AlienState::Exploding {
            self.explosion.pos = self.pct_pos();
            self.explosion.draw(ctx, assets);
        }
    }
}
