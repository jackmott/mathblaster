use ggez::nalgebra as na;

use crate::assets::*;
use crate::ggez_utility::*;


pub struct Crosshair {
}
impl Scalable for Crosshair {
    fn get_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.0,0.0)
    }
    fn get_dimensions(&self) -> (f32, f32) {
        (0.10, 0.1)
    }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32, f32) {
        let img = &assets.crosshair; 
        (img.width() as f32, img.height() as f32)
    }
}
