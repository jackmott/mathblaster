use ggez;
use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use rand::*;

struct Assets {
    blue_ship_img: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> Assets {
        Assets {
            blue_ship_img: graphics::Image::new(ctx, "/blueships1.png").unwrap(),
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

struct Alien {
    operation: Operation,
    speed: f32,
    pos: na::Point2<f32>,
    num1: i32,
    num2: i32,
}

impl Alien {
    fn new(operation: Operation, speed: f32) -> Alien {
        Alien {
            operation: operation,
            speed: speed,
            pos: na::Point2::new(100.0f32, 100.0f32),
            num1: 2,
            num2: 2,
        }
    }

    fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) {
        let sec = dt.as_millis() as f32 / 1000.0;
        self.pos = self.pos + na::Vector2::new(0.0, self.speed * sec);
    }

    fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
        let desired_size = graphics::size(ctx).0 * 0.1;
        let scale = desired_size / (assets.blue_ship_img.width() as f32);
        let params = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .dest(self.pos)
            .scale(na::Vector2::new(scale, scale))
            .offset(na::Point2::new(0.5, 0.5))
            .rotation(3.14159 / 2.0);
        let _ = graphics::draw(ctx, &assets.blue_ship_img, params);
    }
}

struct MainState {
    dt: std::time::Duration,
    aliens: Vec<Alien>,
    assets: Assets,
    levels: Vec<LevelSpec>,
    current_level:usize
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
            aliens: gen_aliens(&levels[0]),
            assets: assets,
            levels: levels,
            current_level: 0
        })
    }
}

fn gen_aliens(level_spec:&LevelSpec) -> Vec<Alien> {    
    let mut aliens = Vec::new();
    let mut rng = rand::thread_rng();
    let op_count = level_spec.operations.len();
    for i in 0 .. 20 {
        let alien = 
            Alien {
                    operation: level_spec.operations[rng.gen_range(0,op_count)],
                    speed: level_spec.speed,
                    pos: na::Point2::new(rng.gen_range(0.0,800.0),rng.gen_range(-(i+1) as f32 *100.0,0.0)),
                    num1: rng.gen_range(0,level_spec.max_number),
                    num2: rng.gen_range(0,level_spec.max_number) //todo with division add some logic
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        for alien in &mut self.aliens {
            alien.draw(ctx, &mut self.assets);         
        }        
        graphics::present(ctx)?;
        Ok(())
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

    let cb = ggez::ContextBuilder::new("super simple", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
