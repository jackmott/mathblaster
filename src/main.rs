use ggez;
use ggez::audio::SoundSource;
use ggez::conf;
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};
use rand::*;
use std::env;
use std::path;

mod alien;
mod assets;
mod explosion;
mod ggez_utility;
mod level;
mod turret;

use crate::alien::*;
use crate::assets::*;
use crate::explosion::*;
use crate::ggez_utility::*;
use crate::level::*;
use crate::turret::*;

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

#[derive(Debug)]
enum GameState {
    StartMenu,
    Playing,
    Dying,
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
    levels: Vec<Level>,
    current_level: usize,
    current_wave: usize,
    turret: Turret,
    target: Option<(usize, f32)>,
    background: Background,
    state: GameState,
    text: TextState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let levels = Level::new();
        let assets = Assets::new(ctx);
        Ok(MainState {
            aliens: gen_aliens(&levels[0].waves[0], &assets.main_font),
            text: TextState {
                dead_text: graphics::Text::new(("You Have Died", assets.title_font, 128.0)),
                won_text: graphics::Text::new(("You Have Won", assets.main_font, 128.0)),
                press_enter: graphics::Text::new(("Press Enter", assets.main_font, 42.0)),
                math_blaster: graphics::Text::new(("Math Blaster", assets.title_font, 128.0)),
            },
            turret: Turret::new(&assets),
            assets: assets,
            levels: levels,
            current_level: 0,
            current_wave: 0,
            target: None,
            background: Background {},
            state: GameState::StartMenu,
            dt: std::time::Duration::new(0, 0),
        })
    }
}

fn gen_aliens(wave: &Wave, font: &graphics::Font) -> Vec<Alien> {
    let mut aliens: Vec<Alien> = Vec::new();
    let mut rng = rand::thread_rng();
    for group in &wave.groups {
        for i in 0..group.num_ships {
            let num1 = rng.gen_range(0, group.max_number);
            let num2 = rng.gen_range(0, group.max_number); //todo with division add some logic

            let (answer, op) = match group.operation {
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
                operation: group.operation,
                speed: group.speed,
                pos: na::Point2::new(x, -(i as i32) as f32 * 0.1),
                text: graphics::Text::new((text, *font, 24.0)),
                answer: answer,
                explosion: Explosion::new(0.0, na::Point2::new(0.0, 0.0)),
                state: AlienState::Alive,
            };
            aliens.push(alien);
        }
    }
    aliens.sort_by(|a, b| b.pos[1].partial_cmp(&a.pos[1]).unwrap());
    aliens
}
impl MainState {
    fn set_level(&mut self, level: usize, wave: usize) {
        self.current_level = level;
        self.current_wave = wave;
        self.target = None;
        let wave = &self.levels[self.current_level].waves[self.current_wave];
        self.aliens = gen_aliens(wave, &self.assets.main_font);
    }
    fn increment_level_wave(&mut self, ctx: &mut Context) {
        //if we were at the last wave already then go to next level
        if self.current_wave + 1 >= self.levels[self.current_level].waves.len() {
            if self.current_level + 1 >= self.levels.len() {
                self.state = GameState::Won;
            } else {
                self.assets.background = graphics::Image::new(ctx, self.levels[self.current_level+1].background_file.clone()).unwrap();
                self.set_level(self.current_level + 1, 0)
            }
        } else {
            self.set_level(self.current_level, self.current_wave + 1);
        }
    }
    fn update_start_menu(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.set_level(0, 0);
            self.turret = Turret::new(&self.assets);
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
    fn update_dying(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        for alien in &mut self.aliens {
            alien.update(ctx, self.dt);
        }
        self.turret.update(ctx, self.dt);
        for splosion in &mut self.turret.explosions {
            splosion.update(ctx, self.dt);
        }
        if self
            .turret
            .explosions
            .iter()
            .all(|splosion| splosion.elapsed - splosion.start_time > splosion.duration)
        {
            self.state = GameState::Dead;
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
                    self.state = GameState::Dying
                };
            }
            None => (),
        }
        if self
            .aliens
            .iter()
            .all(|alien| alien.state == AlienState::Dead)
        {
            self.increment_level_wave(ctx);
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
        let mut title_pos = get_text_center(ctx, &self.text.math_blaster);
        title_pos[1] *= 0.5;
        let _ = graphics::draw(
            ctx,
            &self.text.press_enter,
            graphics::DrawParam::new()
                .color(graphics::Color::from((255, 255, 255, 255)))
                .dest(text_pos),
        );
        let _ = graphics::draw(
            ctx,
            &self.text.math_blaster,
            graphics::DrawParam::new()
                .color(graphics::Color::from((0, 0, 255, 255)))
                .dest(title_pos),
        );
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
        let mut title_pos = get_text_center(ctx, &self.text.won_text);
        title_pos[1] *= 0.5;
        let _ = graphics::draw(
            ctx,
            &self.text.press_enter,
            graphics::DrawParam::new()
                .color(graphics::Color::from((255, 255, 255, 255)))
                .dest(text_pos),
        );
        let _ = graphics::draw(
            ctx,
            &self.text.won_text,
            graphics::DrawParam::new()
                .color(graphics::Color::from((0, 0, 255, 255)))
                .dest(title_pos),
        );
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
        let mut title_pos = get_text_center(ctx, &self.text.dead_text);
        title_pos[1] *= 0.5;
        let _ = graphics::draw(
            ctx,
            &self.text.press_enter,
            graphics::DrawParam::new()
                .color(graphics::Color::from((255, 255, 255, 255)))
                .dest(text_pos),
        );
        let _ = graphics::draw(
            ctx,
            &self.text.dead_text,
            graphics::DrawParam::new()
                .color(graphics::Color::from((0, 0, 255, 255)))
                .dest(title_pos),
        );
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
    fn draw_dying(&mut self, ctx: &mut Context) -> GameResult {
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
        for i in 0..self.turret.explosions.len() {
            let mut pos = self.turret.get_screen_pos(graphics::size(ctx));
            pos[0] += ((10 + i % 2) as f32 / 100.0) * graphics::size(ctx).0;

            self.turret.explosions[i].draw(ctx, &mut self.assets)
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &self.state {
            GameState::StartMenu => self.update_start_menu(ctx),
            GameState::Playing => self.update_playing(ctx),
            GameState::Dying => self.update_dying(ctx),
            GameState::Dead => self.update_dead(ctx),
            GameState::Won => self.update_won(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.state {
            GameState::StartMenu => self.draw_start_menu(ctx),
            GameState::Playing => self.draw_playing(ctx),
            GameState::Dying => self.draw_dying(ctx),
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
