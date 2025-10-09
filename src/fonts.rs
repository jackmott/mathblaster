use ggez::Context;

use crate::assets::FontAssets;

pub struct Fonts {
    pub main: String,
    pub title: String,
    pub number: String,
}

impl Fonts {
    pub fn new(ctx: &mut Context) -> Fonts {
        let main_font = String::from("main_font");
        let title_font = String::from("title_font");
        let number_font = String::from("number_font");
        let fontsdata = FontAssets::new(ctx);

        ctx.gfx.add_font(&main_font, fontsdata.main_font);
        ctx.gfx.add_font(&title_font, fontsdata.title_font);
        ctx.gfx.add_font(&number_font, fontsdata.number_font);

        Fonts {
            main: main_font,
            title: title_font,
            number: number_font,
        }
    }
}
