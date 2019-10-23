use crate::message::*;
use crate::assets::*;

use std::str;
use std::io::{Read};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use ggez::{Context};
use ggez::filesystem;

#[derive(Deserialize,Serialize,Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Deserialize,Serialize)]
    pub struct Level {
    pub waves: Vec<Wave>,
    pub background_file: String,
    pub title: String,
}

#[derive(Deserialize,Serialize)]
pub struct Wave {
    pub groups: Vec<WaveGroup>,
}

#[derive(Deserialize,Serialize)]
pub struct WaveGroup {
    pub speed: f32,
    pub max_number: i32,
    pub operation: Operation,
    pub num_ships: usize,
}

impl Level {
    pub fn push_title(&self, messages: &mut VecDeque<Message>, assets: &Assets) {
           messages.push_back(Message::new(self.title.clone(),2000.0,assets));
    }
    
    pub fn load_from_file(ctx:&mut Context) -> Vec<Level> {
        //todo if any of this fails, call new instead
        let mut file = filesystem::open(ctx, "/levels.json").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap(); 
        serde_json::from_slice(&buffer[..]).unwrap()
    }

    pub fn new() -> Vec<Level> {
        let level = 
        vec![
            //Level 1
            Level {
                title:"Addition Attack!".to_string(),
                background_file: "/spacebg1.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 1.5,
                            max_number: 5,
                            operation: Operation::Add,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 2.5,
                            max_number: 5,
                            operation: Operation::Add,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 1.5,
                            max_number: 10,
                            operation: Operation::Add,
                            num_ships: 5,
                        }],
                    },
                ],
            },
            //Level 2
            Level {
                title: "Subtraction Subterfuge".to_string(),
                background_file: "/spacebg2.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![
                            WaveGroup {
                                speed: 1.5,
                                max_number: 5,
                                operation: Operation::Subtract,
                                num_ships: 10,
                            },
                            WaveGroup {
                                speed: 2.5,
                                max_number: 5,
                                operation: Operation::Add,
                                num_ships: 3,
                            },
                        ],
                    },
                    Wave {
                        groups: vec![
                            WaveGroup {
                                speed: 2.5,
                                max_number: 5,
                                operation: Operation::Subtract,
                                num_ships: 10,
                            },
                            WaveGroup {
                                speed: 3.5,
                                max_number: 5,
                                operation: Operation::Add,
                                num_ships: 3,
                            },
                        ],
                    },
                    Wave {
                        groups: vec![
                            WaveGroup {
                                speed: 1.5,
                                max_number: 10,
                                operation: Operation::Subtract,
                                num_ships: 10,
                            },
                            WaveGroup {
                                speed: 2.5,
                                max_number: 10,
                                operation: Operation::Add,
                                num_ships: 3,
                            },
                        ],
                    },
                ],
            },
        ];

        // todo use this if file not found
        // let serialized = serde_json::to_string_pretty(&level).unwrap();
        level
    }
}
