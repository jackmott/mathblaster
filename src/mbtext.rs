use ggez::nalgebra as na;
use ggez::Context;
use crate::ggez_utility::*;
use ggez::graphics::{self, Color};


pub struct MBText {
    pub text: graphics::Text,
    //pub pos: na::Point2<f32>,        
    pub actual_w:f32,
    pub actual_h:f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl MBText {
    pub fn new(text:String,font:&graphics::Font,color:Color,size:f32,context:&mut Context) -> MBText {
        let text = graphics::Text::new((text, *font, size));
        let dim = text.dimensions(context);
        MBText {
         text: text,
         actual_w: dim.0 as f32,
         actual_h: dim.1 as f32,
         color:color,
         //pos: pos,
         w: dim.0 as f32 /1920.0,
         h: dim.1 as f32 / 1080.0
        }
    }

    pub fn draw(&self,pos: na::Point2<f32>,ctx:&mut Context) {        
        let _ = graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::new()
                .color(self.color)
                .dest(pos)
                .scale(self.scale(graphics::size(ctx)))
        );
    }

    pub fn draw_center(&self,ctx:&mut Context) {
        let pos = self.center(ctx);
        self.draw(pos,ctx);
    }

    pub fn center(&self,ctx: &mut Context) -> na::Point2<f32> {
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
        na::Point2::new(0.0,0.0)
    }

    fn pct_dimensions(&self) -> (f32,f32) {
        (self.w,self.h)
    }
    
    fn src_pixel_dimensions(&self) -> (f32,f32) {
        (self.actual_w,self.actual_h)
    }

}



