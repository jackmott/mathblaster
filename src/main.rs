use ggez;
use ggez::event;
use ggez::graphics::{self,Color};
use ggez::nalgebra as na;
use ggez::{Context,GameResult};
use std::path;
use std::env;

enum AlienKind {
    Add,
    Subtract,
    Multiply,
    Divide
}


struct Alien {
    kind: AlienKind,
    pos: na::Point2<f32>,    
    img: graphics::Image,
    num1: i32,
    num2: i32
}

impl Alien {

    fn new(ctx : &mut Context, kind: AlienKind) -> Alien {       
        Alien {
            kind: kind,
            pos: na::Point2::new(0.0f32,0.0f32),
            img: graphics::Image::new(ctx, "/blueships1.png").unwrap(),
            num1: 2,
            num2: 2,
        }        
    }

    fn update(&mut self, ctx:&mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        graphics::draw(ctx,&self.img,(self.pos,0.0,Color::from((255,255,255,255))))
    }
}


struct MainState {
    alien: Alien
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {        
        Ok( MainState { alien: Alien::new(ctx,AlienKind::Add) } )
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx:&mut Context) -> GameResult {      
        Ok(())
    }

    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        graphics::clear(ctx, [0.1,0.2,0.3,1.0].into());
        self.alien.draw(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = 
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };

    let cb = ggez::ContextBuilder::new("super simple","ggez").add_resource_path(resource_dir);
    let (ctx,event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx,event_loop,state)
}
