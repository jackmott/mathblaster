use ggez::nalgebra as na;

use crate::ggez_utility::*;
use ggez::graphics::{self, Color};
use crate::assets::*;

enum MBTextType {
    Title,
    Equation,
    Main,
}

pub struct MBText {
    pub text: graphics::Text,
    pub pos: na::Point2<f32>,    
    pub desired_w: f32,
    pub desired_h: f32,
    pub actual_w:f32,
    pub actual_h:f32,
}


impl Scalable for MBText {
    fn get_pos(&self) -> na::Point2<f32> {
        self.pos
    }

    fn get_dimensions(&self) -> (f32,f32) {
        (self.w,self.h)
    }
    
    fn get_texture_dimensions(&self, _assets: &Assets) -> (f32,f32) {
        (self.actual_w,self.actual_h)
    }

}



