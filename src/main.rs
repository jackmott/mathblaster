// todo laser draws for one frame too long
// todo consider pulsating/rotating crosshair
//

use ggez;
use ggez::audio::SoundSource;
use ggez::conf;
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self,Color};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};
use rand::*;
use std::env;
use std::path;
use std::collections::VecDeque;

mod message;
mod alien;
mod assets;
mod explosion;
mod ggez_utility;
mod level;
mod turret;
mod crosshair;

use crate::crosshair::*;
use crate::message::*;
use crate::alien::*;
use crate::assets::*;
use crate::explosion::*;
use crate::ggez_utility::*;
use crate::level::*;
use crate::turret::*;

fn get_first_living_alien(aliens: &Vec<Alien>) -> Option<usize> {
    match aliens.iter().enumerate().find(|(_,alien)| alien.state != AlienState::Dead) {
        Some ((index,_)) => Some(index),
        None => None
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
    aliens.sort_by(|a, b| a.pos[0].partial_cmp(&b.pos[0]).unwrap());
    aliens
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

#[derive(Debug)]
enum GameState {
    StartMenu,
    LevelComplete,
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
    level_complete: graphics::Text,
}
struct MainState {
    messages: VecDeque<Message>,
    dt: std::time::Duration,
    aliens: Vec<Alien>,
    assets: Assets,
    levels: Vec<Level>,
    current_level: usize,
    current_wave: usize,
    turret: Turret,
    target: Option<usize>,
    background: Background,
    state: GameState,
    text: TextState,
    lives: usize,
}

impl MainState {
     fn new(ctx: &mut Context) -> GameResult<MainState> {
        let levels = Level::load_from_file(ctx);
        let assets = Assets::new(ctx);
        let mut messages = VecDeque::new();
        messages.push_back(Message::new(levels[0].title.clone(),2000.0,&assets));
        messages.push_back(Message::new("Wave 1".to_string(),2000.0,&assets));
        Ok(MainState {
            messages: messages,
            aliens: gen_aliens(&levels[0].waves[0], &assets.main_font),
            text: TextState {
                dead_text: graphics::Text::new(("You Have Died", assets.title_font, 128.0)),
                won_text: graphics::Text::new(("You Have Won", assets.main_font, 128.0)),
                press_enter: graphics::Text::new(("Press Enter", assets.main_font, 42.0)),
                math_blaster: graphics::Text::new(("Math Blaster", assets.title_font, 128.0)),
                level_complete: graphics::Text::new(("Level Complete!",assets.title_font,128.0))
            },
            turret: Turret::new(&assets),
            assets: assets,
            levels: levels,
            current_level: 0,
            current_wave: 0,
            target: Some(0),
            background: Background {},
            state: GameState::StartMenu,
            dt: std::time::Duration::new(0, 0),
            lives:2
        })        
    }

    fn set_level_wave(&mut self, level: usize, wave: usize,ctx:&mut Context) {
        if level > self.current_level {
            self.state = GameState::LevelComplete;            
            self.assets.background = graphics::Image::new(ctx, self.levels[self.current_level+1].background_file.clone()).unwrap();                
        }
        self.current_level = level;
        self.current_wave = wave;
        self.target = None;
        let wave = &self.levels[self.current_level].waves[self.current_wave];
        self.aliens = gen_aliens(wave, &self.assets.main_font);        
        self.target = Some(0);
    }
    fn increment_level_wave(&mut self, ctx: &mut Context) {
        //if we were at the last wave already then go to next level
        if self.current_wave + 1 >= self.levels[self.current_level].waves.len() {
            if self.current_level + 1 >= self.levels.len() {
                self.state = GameState::Won;
            } else {                                
                self.set_level_wave(self.current_level + 1, 0,ctx)
            }
        } else {
            self.set_level_wave(self.current_level, self.current_wave + 1,ctx);
            self.messages.push_back(Message::new("Wave ".to_string()+&self.current_wave.to_string(),2000.0,&self.assets));
        }
    }
    fn update_start_menu(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.set_level_wave(0, 0,ctx);
            self.lives = 2;
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
    fn update_level_complete(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.state = GameState::Playing;
            self.levels[self.current_level].push_title(&mut self.messages,&self.assets);
            self.messages.push_back(Message::new("Wave 1".to_string(),2000.0,&self.assets));
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
            if self.lives > 0 {
                self.lives -= 1;
                self.turret = Turret::new(&self.assets);
                self.set_level_wave(self.current_level,0,ctx);
                self.state = GameState::Playing;
                self.levels[self.current_level].push_title(&mut self.messages,&self.assets);
            } else {
                self.state = GameState::Dead;
            }
        }
        Ok(())
    }

    fn update_playing(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);

        //update aliens and turret, and message queue
        for alien in &mut self.aliens {
            alien.update(ctx, self.dt);
        }
        self.turret.update(ctx, self.dt);
        if !self.messages.is_empty() {
            self.messages[0].update(self.dt);
            if self.messages[0].elapsed >= self.messages[0].duration {
                let _ = self.messages.pop_front();
            }
        }

        // If there is a target, rotate the turret to it
        match self.target {
            Some(target) if self.aliens[target].state != AlienState::Dead =>  {
                let turret_pos = self.turret.get_pos();
                let turret_vector: na::Vector2<f32> =
                    na::Vector2::new(turret_pos[0], turret_pos[1]);
                let alien_pos = self.aliens[target].get_pos();
                let alien_vector = na::Vector2::new(alien_pos[0], alien_pos[1]);
                let v1 = na::Vector2::new(0.0, -1.0);
                let v2 = alien_vector - turret_vector;
                let mut angle = v2.angle(&v1);
                if alien_pos[0] < 0.5 {
                    angle = -angle;
                }                        
                self.turret.rotation = angle;                    
            },
            Some(_) => self.target = get_first_living_alien(&self.aliens),
            None => (),
        };

        // Find the alien furthest down the screen, if its at the bottom, dead.
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

        //If all aliens are dead, increment the wave/level
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

     fn draw_level_complete(&mut self, ctx: &mut Context) -> GameResult {
        let background_param = graphics::DrawParam::new().scale(
            self.background
                .get_texture_scale(graphics::size(ctx), &self.assets),
        );
        let _ = graphics::draw(ctx, &self.assets.background, background_param);
        let text_pos = get_text_center(ctx, &self.text.press_enter);
        let mut title_pos = get_text_center(ctx, &self.text.level_complete);
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
            &self.text.level_complete,
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

        //Draw the background
        let background_param = graphics::DrawParam::new().scale(
            self.background
                .get_texture_scale(graphics::size(ctx), &self.assets),
        );
        let _ = graphics::draw(ctx, &self.assets.background, background_param);

        // if we have a target, draw the crosshair
        match self.target {
            Some(target) => {
                let alien = &self.aliens[target];

                //draw the crosshair on the target
                let crosshair_pos = to_screen_pos((alien.pos[0],alien.pos[1]),graphics::size(ctx));
                let crosshair = Crosshair{};
                let crosshair_params = graphics::DrawParam::new()
                    .color(Color::from((255, 0, 0, 255)))
                    .dest(crosshair_pos)
                    .scale(crosshair.get_texture_scale(graphics::size(ctx), &self.assets))
                    .offset(na::Point2::new(0.5, 0.5));
                let _ = graphics::draw(ctx,&self.assets.crosshair,crosshair_params);

                //draw the laser if the turret is firing                                   
                match self.turret.state {
                    TurretState::Firing(_) =>
                      {
                        println!("making a laser");
                        let laser = graphics::Mesh::new_line(
                            ctx,
                            &[
                                self.turret.get_screen_pos(graphics::size(ctx)),
                                alien.get_screen_pos(graphics::size(ctx)),
                            ],
                            4.0,
                            graphics::Color::from((255, 0, 0, 255)),
                        )
                        .unwrap();
                        let _ = graphics::draw(ctx, &laser, graphics::DrawParam::default());                
                      },
                      TurretState::Resting => ()
                }
            }
            None => (),
        };

        //draw the aliens, turrets, and messages
        for alien in &mut self.aliens {
            alien.draw(ctx, &mut self.assets);
        }                
        self.turret.draw(ctx, &mut self.assets);
        self.turret.draw_lives(self.lives,ctx,&mut self.assets);
        if !self.messages.is_empty() {
            self.messages[0].draw(ctx);
        }

        graphics::present(ctx)?;
        Ok(())
    }
    fn draw_dying(&mut self, ctx: &mut Context) -> GameResult {
        let background_param = graphics::DrawParam::new().scale(
            self.background
                .get_texture_scale(graphics::size(ctx), &self.assets),
        );
        let _ = graphics::draw(ctx, &self.assets.background, background_param);
        for alien in &mut self.aliens {
            alien.draw(ctx, &mut self.assets);
        }
        self.turret.draw(ctx, &mut self.assets);
        self.turret.draw_lives(self.lives,ctx,&mut self.assets);
        for i in 0..self.turret.explosions.len() {
            let mut pos = self.turret.get_screen_pos(graphics::size(ctx));
            pos[0] += ((10 + i % 2) as f32 / 100.0) * graphics::size(ctx).0;

            self.turret.explosions[i].draw(ctx, &mut self.assets)
        }
        //todo move this into the main draw funcrtion since we always just do this at the end?        
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
            GameState::LevelComplete => self.update_level_complete(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.state {
            GameState::StartMenu => self.draw_start_menu(ctx),
            GameState::Playing => self.draw_playing(ctx),
            GameState::Dying => self.draw_dying(ctx),
            GameState::Dead => self.draw_dead(ctx),
            GameState::Won => self.draw_won(ctx),
            GameState::LevelComplete => self.draw_level_complete(ctx)
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
                Ok(n) => 
                    match self.target {
                        Some(alien_index) if self.aliens[alien_index].answer == n =>
                            {
                                self.aliens[alien_index].state = AlienState::Exploding;
                                let _ = self.assets.explosion_sound.play_detached();
                                self.turret.state = TurretState::Firing(500.0);
                            },
                        _ => () //todo play some kind of sound or show a message that you were wrong?
                    }
                Err(_) => (),
            }
            self.turret.raw_text = "".to_string();
            self.turret.text =
                graphics::Text::new((self.turret.raw_text.clone(), self.assets.main_font, 24.0));
        } else if keycode == KeyCode::Back {
            let _ = self.turret.raw_text.pop();
            self.turret.text =
                graphics::Text::new((self.turret.raw_text.clone(), self.assets.main_font, 24.0));
        } else if keycode == KeyCode::Left {
            match self.target {
                Some(index) if index > 0 =>
                    {
                        let mut new_index = index - 1;
                        while self.aliens[new_index].state == AlienState::Dead && new_index > 0 {
                            new_index -= 1;
                        }
                        if self.aliens[new_index].state != AlienState::Dead {
                            self.target = Some(new_index);
                        } else {
                            self.target = None;
                        }
                    },
                _ => ()
            }
        }
        else if keycode == KeyCode::Right {
            println!("left");
            match self.target {
                Some(index) if index < self.aliens.len() - 1 =>
                    {
                        let mut new_index = index + 1;
                        while self.aliens[new_index].state == AlienState::Dead && new_index <= self.aliens.len() {
                            new_index += 1;
                        }
                        if self.aliens[new_index].state != AlienState::Dead {
                            self.target = Some(new_index);
                        } else {
                            self.target = None;
                        }
                    },
                _ => ()
            }
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
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true),
        );

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    state.assets.explosion_sound.set_volume(0.05);
    state.assets.music.set_repeat(true);
    state.assets.music.set_volume(0.01);
    let _ = state.assets.music.play_detached();
    state.dt = std::time::Duration::new(0, 0);
    println!("about to call run");
    event::run(ctx, event_loop, state)
}
