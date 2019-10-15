// todo
// rotate turret to target
// make two laser beams from actual turret points
// game lose sate
// add more ship images
// multiple levels
// make ships spawn inward a bit
//

use ggez;
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::conf;
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};
use rand::*;
use std::env;
use std::path;

trait Scalable {
    fn get_dimensions(&self) -> (f32, f32);
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32, f32);
    fn get_pos(&self) -> na::Point2<f32>;
    fn get_screen_dimensions(&self, screen_dimensions: (f32, f32)) -> (f32, f32) {
        let (w, h) = self.get_dimensions();
        (w * screen_dimensions.0, h * screen_dimensions.1)
    }
    fn get_texture_scale(
        &self,
        screen_dimensions: (f32, f32),
        assets: &Assets,
    ) -> na::Vector2<f32> {
        let (sw, _) = self.get_screen_dimensions(screen_dimensions);
        let (tw, th) = self.get_texture_dimensions(assets);
        // only use screen width for scaling
        na::Vector2::new(sw / tw, sw / th)
    }
    fn get_screen_pos(&self, screen_dimensions: (f32, f32)) -> na::Point2<f32> {
        let p = self.get_pos();
        na::Point2::new(p[0] * screen_dimensions.0, p[1] * screen_dimensions.1)
    }
}

struct Assets {
    add_ship: graphics::Image,
    sub_ship: graphics::Image,
    mul_ship: graphics::Image,
    div_ship: graphics::Image,
    title_font: graphics::Font,
    main_font: graphics::Font,
    turret: graphics::Image,
    background: graphics::Image,
    explosion: graphics::Image,
    explosion_sound: audio::Source,
    music: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> Assets {
        Assets {
            add_ship: graphics::Image::new(ctx, "/add-ship.png").unwrap(),
            sub_ship: graphics::Image::new(ctx, "/sub-ship.png").unwrap(),
            mul_ship: graphics::Image::new(ctx, "/mul-ship.png").unwrap(),
            div_ship: graphics::Image::new(ctx, "/div-ship.png").unwrap(),
            title_font: graphics::Font::new(ctx,"/title.ttf").unwrap(),
            main_font: graphics::Font::new(ctx, "/main.ttf").unwrap(),
            turret: graphics::Image::new(ctx, "/turret.png").unwrap(),
            background: graphics::Image::new(ctx, "/spacebg1.jpg").unwrap(),
            explosion: graphics::Image::new(ctx, "/explosion.png").unwrap(),
            explosion_sound: audio::Source::new(ctx, "/explosion.wav").unwrap(),
            music: audio::Source::new(ctx, "/music.ogg").unwrap(),
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
    num_ships: usize,
}

struct Explosion {
    duration: f32, //millis
    elapsed: f32,  //millis
    index: usize,
}

fn get_text_center(ctx: &mut Context, text: &graphics::Text) -> na::Point2<f32> {
    let window_dim = graphics::size(ctx);
    let text_dim = text.dimensions(ctx);
    na::Point2::new(
        window_dim.0 / 2.0 - text_dim.0 as f32 / 2.0,
        window_dim.1 / 2.0 - text_dim.1 as f32 / 2.0,
    )
}

impl Explosion {
    fn new() -> Explosion {
        Explosion {
            duration: 500.0,
            elapsed: 0.0,
            index: 0,
        }
    }
    fn get_rect(&self) -> graphics::Rect {
        let index = 15 - self.index; //reverse the order
        let x = index % 4;
        let y = index / 4;
        graphics::Rect::new(
            x as f32 * 64.0 / 255.0,
            y as f32 * 64.0 / 255.0,
            64.0 / 255.0,
            64.0 / 255.0,
        )
    }

    fn update(&mut self, _ctx: &mut Context, dt: std::time::Duration) {
        if self.elapsed <= self.duration {
            self.elapsed += dt.as_millis() as f32;
            let mut index = (self.elapsed / self.duration * 30.0) as i32;
            if index > 15 {
                index = 15 + (15 - index);
            }
            println!("index in update:{}", index);
            self.index = index as usize;
        }
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets, point: na::Point2<f32>) {
        if self.elapsed <= self.duration {
            println!("drawing");
            let param = DrawParam::new()
                .color(Color::from((255, 255, 255, 255)))
                .dest(point - na::Vector2::new(32.0, 32.0))
                .src(self.get_rect());
            let _ = graphics::draw(ctx, &assets.explosion, param);
        }
    }
}
struct Turret {
    rotation: f32,
    raw_text: String,
    text: graphics::Text,
}
impl Scalable for Turret {
    fn get_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.5, 0.9)
    }
    fn get_dimensions(&self) -> (f32, f32) {
        (0.05, 0.05)
    }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32, f32) {
        (assets.turret.width() as f32, assets.turret.height() as f32)
    }
}
impl Turret {
    fn update(&mut self, _ctx: &mut Context, _dt: std::time::Duration) {}

    fn draw(&mut self, ctx: &mut Context, assets: &mut Assets) {
        let param = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .scale(self.get_texture_scale(graphics::size(ctx), assets))
            .offset(na::Point2::new(0.5, 0.5))
            .rotation(self.rotation)
            .dest(self.get_screen_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &assets.turret, param);
        let text_param = DrawParam::new()
            .color(Color::from((255, 255, 255, 255)))
            .dest(self.get_screen_pos(graphics::size(ctx)));
        let _ = graphics::draw(ctx, &self.text, text_param);
    }
}

struct Background {}
impl Scalable for Background {
    fn get_pos(&self) -> na::Point2<f32> {
        na::Point2::new(0.0, 0.0)
    }
    fn get_dimensions(&self) -> (f32, f32) {
        (1.0, 1.0)
    }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32, f32) {
        (
            assets.background.width() as f32,
            assets.background.height() as f32,
        )
    }
    fn get_texture_scale(
        &self,
        screen_dimensions: (f32, f32),
        assets: &Assets,
    ) -> na::Vector2<f32> {
        let (sw, sh) = self.get_screen_dimensions(screen_dimensions);
        let (tw, th) = self.get_texture_dimensions(assets);
        // only use screen width for scaling
        na::Vector2::new(sw / tw, sh / th)
    }
}
#[derive(PartialEq)]
enum AlienState {
    Alive,
    Exploding,
    Dead,
}
struct Alien {
    operation: Operation,
    speed: f32,
    pos: na::Point2<f32>,
    text: graphics::Text,
    answer: i32,
    explosion: Explosion,
    state: AlienState,
}
impl Scalable for Alien {
    fn get_pos(&self) -> na::Point2<f32> {
        self.pos
    }
    fn get_dimensions(&self) -> (f32, f32) {
        (0.06, 0.07)
    }
    fn get_texture_dimensions(&self, assets: &Assets) -> (f32, f32) {
        let img = match self.operation {
            Operation::Add => &assets.add_ship,
            Operation::Subtract => &assets.sub_ship,
            Operation::Multiply => &assets.mul_ship,
            Operation::Divide => &assets.div_ship,
        };

        (img.width() as f32, img.height() as f32)
    }
}
impl Alien {
    fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) {
        if self.state != AlienState::Dead {
            let sec = dt.as_millis() as f32 / 100000.0;
            if self.pos[1] < 0.07 {
                self.pos = self.pos + na::Vector2::new(0.0, self.speed * 3. * sec);
            } else {
                self.pos = self.pos + na::Vector2::new(0.0, self.speed * sec);
            }
            if self.state == AlienState::Exploding {
                self.explosion.update(ctx, dt);
            }
            if self.explosion.elapsed > self.explosion.duration {
                self.state = AlienState::Dead;
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) {
        if self.state != AlienState::Dead {
            if self.explosion.elapsed < self.explosion.duration / 2.0 {
                let params = DrawParam::new()
                    .color(Color::from((255, 255, 255, 255)))
                    .dest(self.get_screen_pos(graphics::size(ctx)))
                    .scale(self.get_texture_scale(graphics::size(ctx), assets))
                    .offset(na::Point2::new(0.5, 0.5))
                    .rotation(3.14159 / 2.0);
                let img = match self.operation {
                    Operation::Add => &assets.add_ship,
                    Operation::Subtract => &assets.sub_ship,
                    Operation::Multiply => &assets.mul_ship,
                    Operation::Divide => &assets.div_ship,
                };
                let _ = graphics::draw(ctx, img, params);

                let tw = self.text.width(ctx) as f32;
                let (sw, sh) = self.get_screen_dimensions(graphics::size(ctx));
                let offsetx = -sw / 2.0 + (sw - tw) / 2.0;
                let offsety = -sh / 1.2;

                let offset = na::Vector2::new(offsetx, offsety);

                let text_param = DrawParam::new()
                    .color(Color::from((255, 255, 255, 255)))
                    .dest(self.get_screen_pos(graphics::size(ctx)) + offset);
                let _ = graphics::draw(ctx, &self.text, text_param);
            }
        }

        if self.state == AlienState::Exploding {
            self.explosion
                .draw(ctx, assets, self.get_screen_pos(graphics::size(ctx)))
        }
    }
}
#[derive(Debug)]
enum GameState {
    StartMenu,
    Playing,
    Dead,
    Won,
}

struct TextState {
    dead_text: graphics::Text,
    won_text: graphics::Text,
    press_enter: graphics::Text,
    math_blaster: graphics::Text,
}
struct MainState {
    dt: std::time::Duration,
    aliens: Vec<Alien>,
    assets: Assets,
    levels: Vec<LevelSpec>,
    current_level: usize,
    turret: Turret,
    target: Option<(usize, f32)>,
    background: Background,
    state: GameState,
    text: TextState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let levels = vec![
            LevelSpec {
                speed: 8.0,
                max_number: 5,
                operations: vec![Operation::Add],
                num_ships: 5,
            },
            LevelSpec {
                speed: 2.5,
                max_number: 5,
                operations: vec![Operation::Add],
                num_ships: 5,
            },
            LevelSpec {
                speed: 1.5,
                max_number: 5,
                operations: vec![Operation::Subtract],
                num_ships: 5,
            },
            LevelSpec {
                speed: 2.5,
                max_number: 5,
                operations: vec![Operation::Subtract],
                num_ships: 5,
            },
            LevelSpec {
                speed: 1.5,
                max_number: 5,
                operations: vec![Operation::Add, Operation::Subtract],
                num_ships: 10,
            },
            LevelSpec {
                speed: 1.5,
                max_number: 10,
                operations: vec![Operation::Add, Operation::Multiply, Operation::Divide],
                num_ships: 5,
            },
        ];

        let assets = Assets::new(ctx);
        Ok(MainState {
            aliens: gen_aliens(&levels[0], &assets.main_font),
            text: TextState {
                dead_text: graphics::Text::new(("You Have Died", assets.title_font, 128.0)),
                won_text: graphics::Text::new(("You Have Won!!", assets.main_font, 128.0)),
                press_enter: graphics::Text::new((
                    "Press Enter",
                    assets.main_font,
                    42.0,
                )),
                math_blaster: graphics::Text::new(("Math Blaster!",assets.title_font,128.0)),
            },
            turret: Turret {
                rotation: 0.0,
                raw_text: "".to_string(),
                text: graphics::Text::new(("", assets.main_font, 24.0)),
            },
            assets: assets,
            levels: levels,
            current_level: 0,
            target: None,
            background: Background {},
            state: GameState::StartMenu,
            dt: std::time::Duration::new(0, 0),
        })
    }
}

fn gen_aliens(level_spec: &LevelSpec, font: &graphics::Font) -> Vec<Alien> {
    let mut aliens: Vec<Alien> = Vec::new();
    let mut rng = rand::thread_rng();
    let op_count = level_spec.operations.len();
    for i in 0..level_spec.num_ships {
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
        let mut x: f32 = rng.gen_range(0.05, 0.95);
        while aliens
            .iter()
            .rev()
            .take(5)
            .any(|alien| (alien.pos[0] - x).abs() < 0.1)
        {
            x = rng.gen_range(0.05, 0.95);
        }
        let alien = Alien {
            operation: operation,
            speed: level_spec.speed,
            pos: na::Point2::new(x, -(i as i32) as f32 * 0.1),
            text: graphics::Text::new((text, *font, 24.0)),
            answer: answer,
            explosion: Explosion::new(),
            state: AlienState::Alive,
        };
        aliens.push(alien);
    }
    aliens
}
impl MainState {
    fn set_level(&mut self,i:usize) {
            self.current_level = i;
            self.target = None;
            if self.current_level >= self.levels.len() {
                self.state = GameState::Won;
            } else {
                let level_spec = &self.levels[self.current_level];
                self.aliens = gen_aliens(level_spec, &self.assets.main_font);
            }
    }

    fn update_start_menu(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.set_level(0);
            self.state = GameState::Playing;            
        }
        Ok(())
    }
    fn update_won(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.state = GameState::StartMenu;
        }
        Ok(())
    }
    fn update_dead(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.state = GameState::StartMenu;
        }
        Ok(())
    }
    fn update_playing(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        for alien in &mut self.aliens {
            alien.update(ctx, self.dt);
        }
        self.turret.update(ctx, self.dt);
        match self.target {
            Some(target) => {
                self.target = if target.1 < 0.0 {
                    println!("removing laser");
                    None
                } else {
                    let turret_pos = self.turret.get_pos();
                    let turret_vector: na::Vector2<f32> =
                        na::Vector2::new(turret_pos[0], turret_pos[1]);
                    let alien_pos = self.aliens[target.0].get_pos();
                    let alien_vector = na::Vector2::new(alien_pos[0], alien_pos[1]);
                    let v1 = na::Vector2::new(0.0, -1.0);
                    let v2 = alien_vector - turret_vector;
                    let mut angle = v2.angle(&v1);
                    if alien_pos[0] < 0.5 {
                        angle = -angle;
                    }
                    println!("angle:{}", angle);
                    self.turret.rotation = angle;
                    //                    let alient_vector = na::Vector2::new(alien.get_pos
                    //                    self.turret.rotation =
                    Some((target.0, target.1 - self.dt.as_millis() as f32))
                };
            }
            None => (),
        };
        match self
            .aliens
            .iter()
            .max_by_key(|alien| (alien.pos[1] * 1000.0) as i32)
        {
            Some(alien) => {
                if alien.pos[1] > 0.9 {
                    println!("seting state to dead");
                    self.state = GameState::Dead
                };
            }
            None => (),
        }
        if self
            .aliens
            .iter()
            .all(|alien| alien.state == AlienState::Dead)
        {
            self.set_level(self.current_level+1);
        }
        Ok(())
    }
    fn draw_start_menu(&mut self, ctx: &mut Context) -> GameResult {
        let background_param = graphics::DrawParam::new().scale(
            self.background
                .get_texture_scale(graphics::size(ctx), &self.assets),
        );
        let _ = graphics::draw(ctx, &self.assets.background, background_param);
        let text_pos = get_text_center(ctx, &self.text.press_enter);
        let mut title_pos = get_text_center(ctx,&self.text.math_blaster);
        title_pos[1] *= 0.5;
        let _ = graphics::draw(
            ctx,
            &self.text.press_enter,
            graphics::DrawParam::new()
                .color(graphics::Color::from((255, 255, 255, 255)))
                .dest(text_pos),
        );
        let _ = graphics::draw(ctx,&self.text.math_blaster,graphics::DrawParam::new().color(graphics::Color::from((0,0,255,255))).dest(title_pos),);
        graphics::present(ctx)?;
        Ok(())
    }
    fn draw_won(&mut self, ctx: &mut Context) -> GameResult {
        let background_param = graphics::DrawParam::new().scale(
            self.background
                .get_texture_scale(graphics::size(ctx), &self.assets),
        );
        let _ = graphics::draw(ctx, &self.assets.background, background_param);
        let text_pos = get_text_center(ctx, &self.text.press_enter);
        let mut title_pos = get_text_center(ctx,&self.text.won_text);
        title_pos[1] *= 0.5;
        let _ = graphics::draw(
            ctx,
            &self.text.press_enter,
            graphics::DrawParam::new()
                .color(graphics::Color::from((255, 255, 255, 255)))
                .dest(text_pos),
        );
        let _ = graphics::draw(ctx,&self.text.won_text,graphics::DrawParam::new().color(graphics::Color::from((0,0,255,255))).dest(title_pos),);
        graphics::present(ctx)?;
        Ok(())
    }
    fn draw_dead(&mut self, ctx: &mut Context) -> GameResult {
        let background_param = graphics::DrawParam::new().scale(
            self.background
                .get_texture_scale(graphics::size(ctx), &self.assets),
        );
        let _ = graphics::draw(ctx, &self.assets.background, background_param);
        let text_pos = get_text_center(ctx, &self.text.press_enter);
        let mut title_pos = get_text_center(ctx,&self.text.dead_text);
        title_pos[1] *= 0.5;
        let _ = graphics::draw(
            ctx,
            &self.text.press_enter,
            graphics::DrawParam::new()
                .color(graphics::Color::from((255, 255, 255, 255)))
                .dest(text_pos),
        );
        let _ = graphics::draw(ctx,&self.text.dead_text,graphics::DrawParam::new().color(graphics::Color::from((0,0,255,255))).dest(title_pos),);
        graphics::present(ctx)?;
        Ok(())
    }
    fn draw_playing(&mut self, ctx: &mut Context) -> GameResult {
        let background_param = graphics::DrawParam::new().scale(
            self.background
                .get_texture_scale(graphics::size(ctx), &self.assets),
        );
        let _ = graphics::draw(ctx, &self.assets.background, background_param);
        match self.target {
            Some(target) => {
                println!("making a laser");
                let laser = graphics::Mesh::new_line(
                    ctx,
                    &[
                        self.turret.get_screen_pos(graphics::size(ctx)),
                        self.aliens[target.0].get_screen_pos(graphics::size(ctx)),
                    ],
                    4.0,
                    graphics::Color::from((255, 0, 0, 255)),
                )
                .unwrap();
                let r = graphics::draw(ctx, &laser, graphics::DrawParam::default());
                println!("err? : {:?}", r);
            }
            None => (),
        };
        for alien in &mut self.aliens {
            alien.draw(ctx, &mut self.assets);
        }
        self.turret.draw(ctx, &mut self.assets);
        graphics::present(ctx)?;
        Ok(())
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &self.state {
            GameState::StartMenu => self.update_start_menu(ctx),
            GameState::Playing => self.update_playing(ctx),
            GameState::Dead => self.update_dead(ctx),
            GameState::Won => self.update_won(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.state {
            GameState::StartMenu => self.draw_start_menu(ctx),
            GameState::Playing => self.draw_playing(ctx),
            GameState::Dead => self.draw_dead(ctx),
            GameState::Won => self.draw_won(ctx),
        }
    }
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("Resized screen to {}, {}", width, height);

        let new_rect = graphics::Rect::new(0.0, 0.0, width as f32, height as f32);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }

    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        if ('0' <= ch && ch <= '9') || ch == '-' {
            println!("text input:{}", ch);
            self.turret.raw_text += &ch.to_string();
            self.turret.text =
                graphics::Text::new((self.turret.raw_text.clone(), self.assets.main_font, 24.0));
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        println!("key up: {:?}", keycode);
        if keycode == KeyCode::Return {
            match self.turret.raw_text.parse::<i32>() {
                Ok(n) => match self
                    .aliens
                    .iter()
                    .position(|alien| alien.answer == n && alien.state == AlienState::Alive)
                {
                    Some(i) => {
                        self.aliens[i].state = AlienState::Exploding;
                        self.target = Some((i, 500.0));
                        let _ = self.assets.explosion_sound.play_detached();
                    }
                    None => (),
                },
                Err(_) => (),
            }
            self.turret.raw_text = "".to_string();
            self.turret.text =
                graphics::Text::new((self.turret.raw_text.clone(), self.assets.main_font, 24.0));
        } else if keycode == KeyCode::Back {
            let _ = self.turret.raw_text.pop();
            self.turret.text =
                graphics::Text::new((self.turret.raw_text.clone(), self.assets.main_font, 24.0));
        }
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
                .fullscreen_type(conf::FullscreenType::True)
                .resizable(true),
        );

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    state.assets.music.set_repeat(true);
    let _ = state.assets.music.play_detached();
    state.dt = std::time::Duration::new(0, 0);
    println!("about to call run");
    event::run(ctx, event_loop, state)
}
