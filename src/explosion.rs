use ggez::audio::SoundSource;
use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra as na;
use ggez::Context;

use crate::assets::*;

pub struct Explosion {
    pub start_time: f32, // millis
    pub duration: f32,   //millis
    pub elapsed: f32,    //millis
    pub index: usize,
    pub pos: na::Point2<f32>,
    pub sound_played: bool,
}
impl Explosion {
    pub fn new(start_time: f32, pos: na::Point2<f32>) -> Explosion {
        Explosion {
            start_time: start_time,
            duration: 500.0,
            elapsed: 0.0,
            index: 0,
            pos: pos,
            sound_played: false,
        }
    }
    pub fn get_rect(&self) -> graphics::Rect {
        let index = 15 - self.index; //reverse the order
        let x = index % 4;
        let y = index / 4;
        graphics::Rect::new(
            x as f32 * 64.0 / 255.0,
            y as f32 * 64.0 / 255.0,
            64.0 / 255.0,
            64.0 / 255.0,
        )
    }

    pub fn update(&mut self, _ctx: &mut Context, dt: std::time::Duration) {
        if self.elapsed - self.start_time <= self.duration {
            self.elapsed += dt.as_millis() as f32;
            if self.elapsed >= self.start_time {
                let mut index = ((self.elapsed - self.start_time) / self.duration * 30.0) as i32;
                if index > 15 {
                    index = 15 + (15 - index);
                }
                self.index = index as usize;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
        if self.elapsed >= self.start_time {
            if !self.sound_played {
                let _ = assets.explosion_sound.play_detached();
                self.sound_played = true;
            }
            if self.elapsed - self.start_time <= self.duration {
                let screen = graphics::size(ctx);
                let param = DrawParam::new()
                    .color(Color::from((255, 255, 255, 255)))
                    .dest(
                        na::Point2::new(self.pos[0] * screen.0, self.pos[1] * screen.1)
                            - na::Vector2::new(32.0, 32.0),
                    )
                    .src(self.get_rect());
                let _ = graphics::draw(ctx, &assets.explosion, param);
            }
        }
    }
}
