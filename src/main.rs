// todo - transition effect when moving to next level?
// todo - transition screen after losing a life?
// todo fix font and display on turret numbers


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
use std::collections::VecDeque;
use std::env;
use std::path;

mod alien;
mod assets;
mod crosshair;
mod explosion;
mod ggez_utility;
mod level;
mod mbtext;
mod message;
mod turret;
mod background;

use crate::background::*;
use crate::alien::*;
use crate::assets::*;
use crate::crosshair::*;
use crate::explosion::*;
use crate::ggez_utility::*;
use crate::level::*;
use crate::mbtext::*;
use crate::message::*;
use crate::turret::*;

fn get_first_living_alien(aliens: &Vec<Alien>) -> Option<usize> {
    aliens
        .iter()
        .position(|alien| alien.state != AlienState::Dead)
}

fn get_lowest_living_alien(aliens: &Vec<Alien>) -> Option<usize> {
    match aliens
        .iter()
        .enumerate()
        .filter(|(_, alien)| alien.state != AlienState::Dead)
        .max_by_key(|(_, alien)| (alien.pos[1] * 1000.0) as i32)
    {
        Some((index, _)) => Some(index),
        None => None,
    }
}

fn gen_aliens(wave: &Wave, assets: &Assets) -> Vec<Alien> {
    let mut aliens: Vec<Alien> = Vec::new();
    let mut rng = rand::thread_rng();
    for group in &wave.groups {
        let alien_img = match group.operation {
            Operation::Add => &assets.add_ship,
            Operation::Subtract => &assets.sub_ship,
            Operation::Multiply => &assets.mul_ship,
            Operation::Divide => &assets.div_ship,
        };
        let alien_img_width = alien_img.width() as f32;
        let alien_img_height = alien_img.height() as f32;

        for i in 0..group.num_ships {
            let (num1, num2) = if group.operation == Operation::Divide {
                let mut a;
                let mut b;
                loop {
                    a = rng.gen_range(group.min_number, group.max_number);
                    b = if a == group.min_number {
                        a
                    } else {
                        rng.gen_range(group.min_number, a)
                    };
                    if b == 0 {
                        continue;
                    }
                    if a % b == 0 {
                        break;
                    }
                }
                (a, b)
            } else {
                (
                    rng.gen_range(group.min_number, group.max_number),
                    rng.gen_range(group.min_number, group.max_number),
                )
            };

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
                text: graphics::Text::new((text, assets.number_font, 24.0)),
                answer: answer,
                explosion: Explosion::new(0.0, na::Point2::new(0.0, 0.0)),
                state: AlienState::Alive,
                src_pixel_width: alien_img_width,
                src_pixel_height: alien_img_height,
            };
            aliens.push(alien);
        }
    }
    aliens.sort_by(|a, b| a.pos[0].partial_cmp(&b.pos[0]).unwrap());
    aliens
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
    dead_text: MBText,
    won_text: MBText,
    press_enter: MBText,
    math_blaster: MBText,
    level_complete: MBText,
    level_names: Vec<MBText>,
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
    crosshair: Crosshair,
    level_selection: usize,
    up_key: Option<KeyCode>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let levels = Level::load_from_file();
        let assets = Assets::new(ctx,levels[0].background_file.clone());
        let messages = VecDeque::new();
        let aliens = gen_aliens(&levels[0].waves[0], &assets);
        let target = get_lowest_living_alien(&aliens);

        Ok(MainState {
            messages: messages,
            aliens: aliens,
            text: TextState {
                dead_text: MBText::new(
                    "You Have Died".to_string(),
                    &assets.title_font,
                    BLUE,
                    128.0,
                    ctx,
                ),
                won_text: MBText::new(
                    "You Win!".to_string(),
                    &assets.main_font,
                    BLUE,
                    128.0,
                    ctx,
                ),
                press_enter: MBText::new(
                    "Press Enter".to_string(),
                    &assets.main_font,
                    WHITE,
                    64.0,
                    ctx,
                ),
                math_blaster: MBText::new(
                    "Math Blaster".to_string(),
                    &assets.title_font,
                    BLUE,
                    128.0,
                    ctx,
                ),
                level_complete: MBText::new(
                    "Level Complete!".to_string(),
                    &assets.title_font,
                    BLUE,
                    128.0,
                    ctx,
                ),
                level_names: levels
                    .iter()
                    .map(|level| {
                        MBText::new_blink(
                            level.title.clone(),
                            &assets.main_font,
                            WHITE,
                            GRAY,
                            64.0,
                            ctx,
                        )
                    })
                    .collect(),
            },
            turret: Turret::new(&assets),
            levels: levels,
            current_level: 0,
            current_wave: 0,
            target: target,
            background: Background {
                src_pixel_width: assets.background.width() as f32,
                src_pixel_height: assets.background.height() as f32,
                stars1_pos: 0.0,
                stars2_pos: 0.0,                
            },
            state: GameState::StartMenu,
            dt: std::time::Duration::new(0, 0),
            lives: 2,
            crosshair: Crosshair {
                elapsed: 0,
                src_pixel_width: assets.crosshair.width() as f32,
                src_pixel_height: assets.crosshair.height() as f32,
            },
            assets: assets,
            level_selection: 0,
            up_key: None,
        })
    }

    fn set_level_wave(&mut self, level: usize, wave: usize) {
        if level > self.current_level {
            self.state = GameState::LevelComplete;
            let _ = self.assets.clap_sound.play_detached();                          
        }
        self.current_level = level;
        self.current_wave = wave;
        self.target = None;
        let wave = &self.levels[self.current_level].waves[self.current_wave];
        self.aliens = gen_aliens(wave, &self.assets);
        self.target = get_lowest_living_alien(&self.aliens);
    }
    fn increment_level_wave(&mut self, ctx: &mut Context) {
        //if we were at the last wave already then go to next level
        if self.current_wave + 1 >= self.levels[self.current_level].waves.len() {
            //unlock the next level and save the json
            self.levels[self.current_level+1].unlocked = true;
            Level::save_levels(&self.levels);
            if self.current_level + 1 >= self.levels.len() {
                self.state = GameState::Won;
            } else {
                self.set_level_wave(self.current_level + 1, 0)
            }
        } else {
            self.set_level_wave(self.current_level, self.current_wave + 1);
            self.messages.push_back(Message::new(
                "Wave Eliminated!".to_string(),
                2000.0,
                &self.assets,
                ctx,
            ));
            self.messages.push_back(Message::new(
                "Wave ".to_string() + &(self.current_wave + 1).to_string(),
                2000.0,
                &self.assets,
                ctx,
            ));
        }
    }
    fn update_start_menu(&mut self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.set_level_wave(self.level_selection, 0);
            self.messages.push_back(Message::new(
                self.levels[self.level_selection].title.clone(),
                2000.0,
                &self.assets,
                ctx,
            ));
            self.messages.push_back(Message::new(
                "Wave 1".to_string(),
                2000.0,
                &self.assets,
                ctx,
            ));
            self.lives = 2;
            self.turret = Turret::new(&self.assets);
            self.state = GameState::Playing;
        }

        let unlocked_count = self.levels.iter().filter(|level| level.unlocked).count();

        if let Some(keycode) = self.up_key {
            //we are dealing with a keycode so clear it
            self.up_key = None;
            if keycode == KeyCode::Down {
                self.level_selection = (self.level_selection + 1) % unlocked_count;
            } else if keycode == KeyCode::Up {
                self.level_selection = if self.level_selection == 0 {
                    unlocked_count-1
                } else {
                    self.level_selection - 1
                };
            }
        }

        for level_name in &mut self.text.level_names {
            level_name.update(self.dt)
        }
    }
    fn update_won(&mut self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.state = GameState::StartMenu;
        }
    }
    fn update_level_complete(&mut self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.assets.background = graphics::Image::new(
                ctx,
                self.levels[self.current_level].background_file.clone(),
            )
            .unwrap();
            self.state = GameState::Playing;
            self.levels[self.current_level].push_title(&mut self.messages, &self.assets, ctx);
            self.messages.push_back(Message::new(
                "Wave 1".to_string(),
                2000.0,
                &self.assets,
                ctx,
            ));
        }
    }
    fn update_dead(&mut self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.state = GameState::StartMenu;
        }
    }
    fn update_dying(&mut self, ctx: &mut Context) {
        for alien in &mut self.aliens {
            alien.update(&mut self.turret, ctx, self.dt);
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
                self.set_level_wave(self.current_level, 0);
                self.state = GameState::Playing;
                self.levels[self.current_level].push_title(&mut self.messages, &self.assets, ctx);
            } else {
                self.state = GameState::Dead;
            }
        }
    }

    fn update_playing(&mut self, ctx: &mut Context) {
        self.background.update(self.dt);
        self.crosshair.update(self.dt);
        if let Some(keycode) = self.up_key {
            //we are going to deal with the up_key so clear it out
            self.up_key = None;
            if keycode == KeyCode::Return {
                match self.turret.raw_text.parse::<i32>() {
                    Ok(n) => match self.target {
                        Some(alien_index) if self.aliens[alien_index].answer == n => {
                            self.aliens[alien_index].state = AlienState::Exploding;
                            let _ = self.assets.explosion_sound.play_detached();
                            self.turret.state = TurretState::Firing;
                        }
                        _ => (), //todo play some kind of sound or show a message that you were wrong?
                    },
                    Err(_) => (),
                }
                self.turret.raw_text = "".to_string();
                self.turret.text = graphics::Text::new((
                    self.turret.raw_text.clone(),
                    self.assets.main_font,
                    24.0,
                ));
            } else if keycode == KeyCode::Back {
                let _ = self.turret.raw_text.pop();
                self.turret.text = graphics::Text::new((
                    self.turret.raw_text.clone(),
                    self.assets.main_font,
                    24.0,
                ));
            } else if keycode == KeyCode::Left {
                match self.target {
                    Some(index) => {
                        if self
                            .aliens
                            .iter()
                            .any(|alien| alien.state == AlienState::Alive && alien.pos[1] >= 0.0)
                        {
                            let mut i = if index == 0 {
                                self.aliens.len() - 1
                            } else {
                                index - 1
                            };
                            while self.aliens[i].state != AlienState::Alive
                                || self.aliens[i].pos[1] < 0.0
                            {
                                i = if i == 0 { self.aliens.len() - 1 } else { i - 1 }
                            }
                            self.target = Some(i);
                        }
                    }
                    _ => (),
                }
            } else if keycode == KeyCode::Right {
                println!("left");
                match self.target {
                    Some(index) => {
                        if self
                            .aliens
                            .iter()
                            .any(|alien| alien.state == AlienState::Alive && alien.pos[1] > 0.0)
                        {
                            let mut i = (index + 1) % self.aliens.len();
                            while self.aliens[i].state != AlienState::Alive
                                || self.aliens[i].pos[1] < 0.0
                            {
                                i = (i + 1) % self.aliens.len();
                            }
                            self.target = Some(i);
                        }
                    }

                    _ => (),
                }
            }
        }

        //update aliens and turret, and message queue
        for alien in &mut self.aliens {
            alien.update(&mut self.turret, ctx, self.dt);
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
            Some(target) if self.aliens[target].state != AlienState::Dead => {
                let turret_pos = self.turret.pct_pos();
                let turret_vector: na::Vector2<f32> =
                    na::Vector2::new(turret_pos[0], turret_pos[1]);
                let alien_pos = self.aliens[target].pct_pos();
                let alien_vector = na::Vector2::new(alien_pos[0], alien_pos[1]);
                let v1 = na::Vector2::new(0.0, -1.0);
                let v2 = alien_vector - turret_vector;
                let mut angle = v2.angle(&v1);
                if alien_pos[0] < 0.5 {
                    angle = -angle;
                }
                self.turret.rotation = angle;
            }
            Some(_) => self.target = get_lowest_living_alien(&self.aliens),
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
    }
    fn draw_start_menu(&mut self, ctx: &mut Context) {
        self.background.draw(ctx, &self.assets);        
        let mut title_pos = self.text.math_blaster.center(ctx);
        title_pos[1] *= 0.5;
        self.text.math_blaster.draw(title_pos, ctx);

        let window_dimension = graphics::size(ctx);
        let mut y = 0.4 * window_dimension.1 as f32;
        for (i, level_name) in self.text.level_names.iter().enumerate() {
            let vertical_size = level_name.dest_pixel_dimensions(window_dimension).1;
            let mut center = level_name.center(ctx);
            center[1] = y;
            if i == self.level_selection {
                level_name.draw(center, ctx);
            } else if self.levels[i].unlocked {
                level_name.draw_color(center, GRAY, ctx);
            } else {
                level_name.draw_color(center, DARK_GRAY, ctx);
            }
            y += vertical_size * 1.075;
        }
    }
    fn draw_won(&mut self, ctx: &mut Context) {
        self.background.draw(ctx, &self.assets);      
        let mut title_pos = self.text.won_text.center(ctx);
        title_pos[1] *= 0.5;
        self.text.press_enter.draw_center(ctx);
        self.text.won_text.draw(title_pos, ctx);
    }

    fn draw_level_complete(&mut self, ctx: &mut Context) {
        self.background.draw(ctx, &self.assets);      
        let mut title_pos = self.text.level_complete.center(ctx);
        title_pos[1] *= 0.5;
        self.text.press_enter.draw_center(ctx);
        self.text.level_complete.draw(title_pos, ctx);
    }

    fn draw_dead(&mut self, ctx: &mut Context) {
        self.background.draw(ctx, &self.assets);      
        let mut title_pos = self.text.dead_text.center(ctx);
        title_pos[1] *= 0.5;
        self.text.press_enter.draw_center(ctx);
        self.text.dead_text.draw(title_pos, ctx);
    }
    fn draw_playing(&mut self, ctx: &mut Context) {
        //Draw the background
       self.background.draw(ctx, &self.assets);      

        // if we have a target, draw the crosshair
        match self.target {
            Some(target) => {
                let alien = &self.aliens[target];

                //draw the crosshair on the target
                let crosshair_pos =
                    to_screen_pos((alien.pos[0], alien.pos[1]), graphics::size(ctx));
                self.crosshair.draw(crosshair_pos, ctx, &self.assets);
                //draw the laser if the turret is firing
                match self.turret.state {
                    TurretState::Firing => {
                        let laser = graphics::Mesh::new_line(
                            ctx,
                            &[
                                self.turret.pixel_pos(graphics::size(ctx)),
                                alien.pixel_pos(graphics::size(ctx)),
                            ],
                            4.0,
                            graphics::Color::from((255, 0, 0, 255)),
                        )
                        .unwrap();
                        let _ = graphics::draw(ctx, &laser, graphics::DrawParam::default());
                    }
                    TurretState::Resting => (),
                }
            }
            None => (),
        };

        //draw the aliens, turrets, and messages
        for alien in &mut self.aliens {
            alien.draw(ctx, &mut self.assets);
        }
        self.turret.draw(ctx, &mut self.assets);
        self.turret.draw_lives(self.lives, ctx, &mut self.assets);
        if !self.messages.is_empty() {
            self.messages[0].draw(ctx);
        }
    }
    fn draw_dying(&mut self, ctx: &mut Context) {
        self.background.draw(ctx, &self.assets);      
        for alien in &mut self.aliens {
            alien.draw(ctx, &mut self.assets);
        }
        self.turret.draw(ctx, &mut self.assets);
        self.turret.draw_lives(self.lives, ctx, &mut self.assets);
        for i in 0..self.turret.explosions.len() {
            let mut pos = self.turret.pixel_pos(graphics::size(ctx));
            pos[0] += ((10 + i % 2) as f32 / 100.0) * graphics::size(ctx).0;

            self.turret.explosions[i].draw(ctx, &mut self.assets)
        }
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        match &self.state {
            GameState::StartMenu => self.update_start_menu(ctx),
            GameState::Playing => self.update_playing(ctx),
            GameState::Dying => self.update_dying(ctx),
            GameState::Dead => self.update_dead(ctx),
            GameState::Won => self.update_won(ctx),
            GameState::LevelComplete => self.update_level_complete(ctx),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        match &self.state {
            GameState::StartMenu => self.draw_start_menu(ctx),
            GameState::Playing => self.draw_playing(ctx),
            GameState::Dying => self.draw_dying(ctx),
            GameState::Dead => self.draw_dead(ctx),
            GameState::Won => self.draw_won(ctx),
            GameState::LevelComplete => self.draw_level_complete(ctx),
        }
        graphics::present(ctx)?;
        Ok(())
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
        self.up_key = Some(keycode);
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
    //state.assets.music.set_volume(0.07);
    let _ = state.assets.music.play_detached();
    state.dt = std::time::Duration::new(0, 0);
    println!("about to call run");
    event::run(ctx, event_loop, state)
}
