use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context,GameResult};


enum AlienType {
    Add,
    Subtract,
    Multiply,
    Divide
}


struct Alien {
    type: AlienType,
    pos: na::Point2,
    img: graphics::Image,
    num1: i32,
    num2: i32
}


struct MainState {
    alien: Alien
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x:0.0};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx:&mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        graphics::clear(ctx, [0.1,0.2,0.3,1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0,0.0),
            100.0,
            2.0,
            graphics::WHITE,
            )?;
        graphics::draw(ctx,&circle,(na::Point2::new(self.pos_x,380.0),))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super simple","ggez");
    let (ctx,event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx,event_loop,state)
}
