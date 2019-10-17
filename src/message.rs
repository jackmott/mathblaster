use ggez::graphics::{self};
use crate::assets::*;

pub struct Message {
    text:graphics::Text,
    duration:f32,
    elapsed:f32
}

impl Message {
    pub fn new(text:String,duration:f32,assets:&Assets) -> Message {
        Message {
            text: graphics::Text::new((text, assets.main_font, 128.0)),
            duration: duration,
            elapsed:0.0
        }
    }
}