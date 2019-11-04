use crate::ggez_utility::*;
use ggez::nalgebra as na;
use ggez::Context;
use crate::assets::*;
use ggez::graphics::{DrawParam};
use ggez::graphics;

pub struct Background {
    pub src_pixel_width: f32,
    pub src_pixel_height: f32,
    pub stars1_pos: f32,
    pub stars2_pos: f32,
}

impl Background {
     pub fn update(&mut self, dt: std::time::Duration) {
        //update the parallax stars are different rates
        let t = dt.as_millis() as f32;
        self.stars1_pos = (self.stars1_pos + t/8000.0) % 1.0;
        self.stars2_pos = (self.stars2_pos + t/16000.0) % 1.0;
    }

    pub fn draw(&mut self,ctx: &mut Context, assets: &Assets) {
       let background_param =
            graphics::DrawParam::new().scale(self.scale(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &assets.background, background_param);

        
        let w = (assets.stars1.width() / 1) as usize;
        let h = (assets.stars1.height() / 1) as usize;
        let (screen_wf,screen_hf) = graphics::size(ctx);
        let screen_w = screen_wf as i32;
        let screen_h = screen_hf as i32;
        let pixel_offset = (self.stars1_pos * h as f32) as usize;

        

        //stars 1       
        let mut y = 0;
        while y < screen_h {   
        
            // h = 100
            //star_pos = .99
            //pixel_offset = 99
            //y_start = 1
            //y_pct = 1%
            //delta = 99

            //start_pos = 0
            // pixel_offset = 0
            //y_start = 0
            // delta = 100


            println!("startpos:{}",self.stars1_pos);
            let (y_pct,delta) = 
                if y == 0 {
                    let y_start = (h-pixel_offset)%h;                                
                    (y_start as f32/h as f32,if pixel_offset == 0 { h } else {pixel_offset})
                } else {
                    (0.0,h)
                };
            let rect = graphics::Rect::new(0.0,y_pct,1.0,1.0-y_pct);
            if y == 0 && self.stars1_pos > 0.9 {
                println!("before rect:{:?}",rect);
            }
            if y == 0 && self.stars1_pos < 0.01 {
                println!("after  rect:{:?}",rect);
            }
            for x in (0 .. screen_w).step_by(w) {                                              
                let star_param = DrawParam::new().src(rect).dest(na::Point2::new(x as f32,y as f32)).scale(na::Vector2::new(1.0,1.0));
                let _ = graphics::draw(ctx,&assets.stars1,star_param);                
            }

            y += delta as i32;
        }        

                               
/*
        //stars 2
        let w = (assets.stars1.width() / 1) as usize;
        let h = (assets.stars1.height() / 1) as usize;
        let start_y = (self.stars2_pos*screen_hf) as i32;        
        for y in (start_y .. screen_h).step_by(h) {
            for x in (0 .. screen_w).step_by(w) {                
                let star_param = DrawParam::new().dest(na::Point2::new(x as f32,y as f32)).scale(na::Vector2::new(1.0,1.0));
                let _ = graphics::draw(ctx,&assets.stars2,star_param);                
            }
        }        

        let remaining_y = screen_h - start_y;
        let new_start_y = remaining_y % h as i32;

        for y in (-new_start_y .. start_y).step_by(h) {
             for x in (0 .. screen_w).step_by(w) {                
                let star_param = DrawParam::new().dest(na::Point2::new(x as f32,y as f32)).scale(na::Vector2::new(1.0,1.0));
                let _ = graphics::draw(ctx,&assets.stars2,star_param);                
            }
        }
        */
        
    }
}

impl Scalable for Background {
    fn pct_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.0, 0.0)
    }
    fn pct_dimensions(&self) -> (f32, f32) {
        (1.0, 1.0)
    }
    fn src_pixel_dimensions(&self) -> (f32, f32) {
        (self.src_pixel_width, self.src_pixel_height)
    }
    fn scale(&self, screen_dimensions: (f32, f32)) -> na::Vector2<f32> {
        let (sw, sh) = self.dest_pixel_dimensions(screen_dimensions);
        let (tw, th) = self.src_pixel_dimensions();
        // only use screen width for scaling
        na::Vector2::new(sw / tw, sh / th)
    }
}