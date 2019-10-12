use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};
use rand::*;
use std::env;
use std::path;



trait Scalable {
    fn get_dimensions(&self) -> (f32,f32);
    fn get_texture_dimensions(&self,assets: &Assets) -> (f32,f32);
    fn get_pos(&self) -> na::Point2<f32>;
    fn get_screen_dimensions(&self,screen_dimensions:(f32,f32)) -> (f32,f32) {
        let (w,h) = self.get_dimensions();
        (w*screen_dimensions.0,h*screen_dimensions.1)
    }
    fn get_texture_scale(&self,screen_dimensions:(f32,f32),assets: &Assets) -> na::Vector2<f32> {
        let (sw,_) = self.get_screen_dimensions(screen_dimensions);
        let (tw,th) = self.get_texture_dimensions(assets);
        // only use screen width for scaling
        na::Vector2::new(sw/tw,sw/th)
    }
    fn get_screen_pos(&self,screen_dimensions:(f32,f32)) -> na::Point2<f32> {
        let p = self.get_pos();
        na::Point2::new(p[0] * screen_dimensions.0, p[1] * screen_dimensions.1)
    }
}


struct Assets {
    blue_ship_img: graphics::Image,
    num_font: graphics::Font,
    turret: graphics::Image,
    background: graphics::Image,
    explosion: graphics::Image
}

impl Assets {
    fn new(ctx: &mut Context) -> Assets {
        Assets {
            blue_ship_img: graphics::Image::new(ctx, "/blueships1.png").unwrap(),
            num_font: graphics::Font::new(ctx, "/dejamono.ttf").unwrap(),
            turret: graphics::Image::new(ctx, "/turret.png").unwrap(),
            background: graphics::Image::new(ctx,"/background.jpg").unwrap(),
            explosion: graphics::Image::new(ctx,"/explosion.png").unwrap()
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct LevelSpec {
    speed: f32,
    max_number: i32,
    operations: Vec<Operation>,
}


struct Explosion {
    duration: f32, //millis
    elapsed: f32, //millis
    index: usize
}
impl Explosion {
    fn new() -> Explosion {
        Explosion {
            duration: 500.0,
            elapsed: 0.0,
            index: 0
        }
    }
    fn get_rect(&self) -> graphics::Rect {
        let index = 15 - self.index; //reverse the order
        let x = index % 4;
        let y = index / 4;
        println!("index: {} rectx:{} recty{}",index,x,y);
        graphics::Rect::new(x as f32 *64.0 / 255.0,y as f32 *64.0 / 255.0,64.0 / 255.0,64.0 / 255.0)
    }

    
    fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) {
        if self.elapsed <= self.duration {
            self.elapsed += dt.as_millis() as f32;
            println!("duration: {} elapsed: {}",self.duration,self.elapsed);
            let mut index = (self.elapsed/self.duration * 30.0) as i32;
            if index > 15 {
                index = 15 + (15-index);
            }
            println!("index in update:{}",index);
            self.index = index as usize;
        }
    }
    
     fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
         if self.elapsed <= self.duration {
            println!("drawing");
            let param = DrawParam::new()
                .color(Color::from((255, 255, 255, 255)))
                .dest(na::Point2::new(0.0,0.0))
                .src(self.get_rect());
            let _ = graphics::draw(ctx, &assets.explosion, param);
         }
     }   
}
struct Turret {
    rotation: f32,
    raw_text: String,
    text: graphics::Text
}
impl Scalable for Turret {
    fn get_pos(&self) -> na::Point2<f32> { na::Point2::new(0.5,0.9) }
    fn get_dimensions(&self) -> (f32,f32) { (0.05,0.05) }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32,f32) { (assets.turret.width() as f32,assets.turret.height() as f32) }
}
impl Turret {
    fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) {
        
    }

    fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
        let param = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .scale(self.get_texture_scale(graphics::size(ctx),assets))
            .offset(na::Point2::new(0.5,0.5))
            .rotation(self.rotation)
            .dest(self.get_screen_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &assets.turret, param);
        let text_param = DrawParam::new()
        .color(Color::from((255,255,255,255)))
        .dest(self.get_screen_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx,&self.text,text_param);
    }
}

struct Background {}
impl Scalable for Background {
    fn get_pos(&self) -> na::Point2<f32> { na::Point2::new(0.0,0.0)}
    fn get_dimensions(&self) -> (f32,f32) { (1.0,1.0) }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32,f32) { (assets.background.width() as f32,assets.background.height() as f32) }
    fn get_texture_scale(&self,screen_dimensions:(f32,f32),assets: &Assets) -> na::Vector2<f32> {
        let (sw,sh) = self.get_screen_dimensions(screen_dimensions);
        let (tw,th) = self.get_texture_dimensions(assets);
        // only use screen width for scaling
        na::Vector2::new(sw/tw,sh/th)
    }
}

struct Alien {
    operation: Operation,
    speed: f32,
    pos: na::Point2<f32>,
    text: graphics::Text,
    answer: i32,
}
impl Scalable for Alien {
    fn get_pos(&self) -> na::Point2<f32> { self.pos }
    fn get_dimensions(&self) -> (f32,f32) { (0.06,0.07) }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32,f32) { (assets.blue_ship_img.width() as f32,assets.blue_ship_img.height() as f32) }
}
impl Alien {
    fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) {
        let sec = dt.as_millis() as f32 / 100000.0;
        self.pos = self.pos + na::Vector2::new(0.0, self.speed * sec);
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) {
        let params = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .dest(self.get_screen_pos(
                graphics::size(ctx)
            ))
            .scale(self.get_texture_scale(graphics::size(ctx),assets))
            .offset(na::Point2::new(0.5, 0.5))
            .rotation(3.14159 / 2.0);
        let _ = graphics::draw(ctx, &assets.blue_ship_img, params);

        let tw = self.text.width(ctx) as f32;
        let (sw,sh) = self.get_screen_dimensions(graphics::size(ctx));
        let offsetx = -sw/2.0 + (sw-tw)/2.0;
        let offsety = -sh/1.2;

        let offset = na::Vector2::new(offsetx,offsety);

        let text_param = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .dest(self.get_screen_pos(
                graphics::size(ctx)
            )+offset);
        let _ = graphics::draw(ctx, &self.text, text_param);
    }
}

struct MainState {
    dt: std::time::Duration,
    aliens: Vec<Alien>,
    assets: Assets,
    levels: Vec<LevelSpec>,
    current_level: usize,
    turret: Turret,
    background: Background,
    explosion: Explosion
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let levels = vec![
            LevelSpec {
                speed: 10.0,
                max_number: 5,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 5,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 5,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 10,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 10,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 10,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 15,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 15,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 15,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 20,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 20,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 20,
                operations: vec![Operation::Add],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 5,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 5,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 5,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 10,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 10,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 10,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 15,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 15,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 15,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 20,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 20,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 20,
                operations: vec![Operation::Subtract],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 5,
                operations: vec![Operation::Multiply],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 5,
                operations: vec![Operation::Multiply],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 5,
                operations: vec![Operation::Multiply],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 10,
                operations: vec![Operation::Multiply],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 10,
                operations: vec![Operation::Multiply],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 10,
                operations: vec![Operation::Multiply],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 5,
                operations: vec![Operation::Divide],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 5,
                operations: vec![Operation::Divide],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 5,
                operations: vec![Operation::Divide],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 10,
                operations: vec![Operation::Divide],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 10,
                operations: vec![Operation::Divide],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 10,
                operations: vec![Operation::Divide],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 5,
                operations: vec![
                    Operation::Add,
                    Operation::Subtract,
                    Operation::Multiply,
                    Operation::Divide,
                ],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 5,
                operations: vec![
                    Operation::Add,
                    Operation::Subtract,
                    Operation::Multiply,
                    Operation::Divide,
                ],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 5,
                operations: vec![
                    Operation::Add,
                    Operation::Subtract,
                    Operation::Multiply,
                    Operation::Divide,
                ],
            },
            LevelSpec {
                speed: 10.0,
                max_number: 10,
                operations: vec![
                    Operation::Add,
                    Operation::Subtract,
                    Operation::Multiply,
                    Operation::Divide,
                ],
            },
            LevelSpec {
                speed: 20.0,
                max_number: 10,
                operations: vec![
                    Operation::Add,
                    Operation::Subtract,
                    Operation::Multiply,
                    Operation::Divide,
                ],
            },
            LevelSpec {
                speed: 40.0,
                max_number: 10,
                operations: vec![
                    Operation::Add,
                    Operation::Subtract,
                    Operation::Multiply,
                    Operation::Divide,
                ],
            },
        ];

        let assets = Assets::new(ctx);
        Ok(MainState {
            dt: std::time::Duration::new(0, 0),
            aliens: gen_aliens(&levels[0], &assets.num_font),
            turret: Turret { rotation: 0.0, raw_text:"".to_string(),text: graphics::Text::new(("",assets.num_font,24.0)) },
            assets: assets,
            levels: levels,
            current_level: 0,
            background: Background{},
            explosion: Explosion::new()
        })
    }
}

fn gen_aliens(level_spec: &LevelSpec, font: &graphics::Font) -> Vec<Alien> {
    let mut aliens = Vec::new();
    let mut rng = rand::thread_rng();
    let op_count = level_spec.operations.len();
    for i in 0..20 {
        let operation = level_spec.operations[rng.gen_range(0, op_count)];
        let num1 = rng.gen_range(0, level_spec.max_number);
        let num2 = rng.gen_range(0, level_spec.max_number); //todo with division add some logic
        let (answer, op) = match operation {
            Operation::Add => (num1 + num2, "+"),
            Operation::Subtract => (num1 - num2, "-"),
            Operation::Multiply => (num1 * num2, "x"),
            Operation::Divide => (num1 / num2, "/"),
        };
        let text = num1.to_string() + op + &num2.to_string();
        let alien = Alien {
            operation: operation,
            speed: level_spec.speed,
            pos: na::Point2::new(rng.gen_range(0.0, 1.0), -i as f32 * 0.1),
            text: graphics::Text::new((text, *font, 24.0)),
            answer: answer,
        };
        aliens.push(alien);
    }
    aliens
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        for alien in &mut self.aliens {
            alien.update(ctx, self.dt);
        }
        self.turret.update(ctx, self.dt);
        self.explosion.update(ctx,self.dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_param = graphics::DrawParam::new().scale(self.background.get_texture_scale(graphics::size(ctx),&self.assets));
        let _ = graphics::draw(ctx,&self.assets.background,background_param);
        for alien in &mut self.aliens {
            alien.draw(ctx, &mut self.assets);
        }
        self.turret.draw(ctx, &mut self.assets);
        self.explosion.draw(ctx,&mut self.assets);
        graphics::present(ctx)?;
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("Resized screen to {}, {}", width, height);

        let new_rect = graphics::Rect::new(0.0, 0.0, width as f32, height as f32);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
    
    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        println!("text input:{}",ch);        
        self.turret.raw_text += &ch.to_string();
        self.turret.text = graphics::Text::new((self.turret.raw_text.clone(),self.assets.num_font,24.0));
        self.explosion.elapsed = 0.0;
    }


}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("super simple", "ggez")
        .add_resource_path(resource_dir)
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true),
        );

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
