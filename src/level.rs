use crate::message::*;
use crate::assets::*;

use std::str;
use std::io::{Read,Write};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use ggez::{Context};
use std::fs::File;
use std::io::prelude::*;

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
    

    pub fn load_from_file() -> Vec<Level> {
        //if any of this fails, call new instead
        fn load_helper() -> Result<Vec<Level>,String> {
            let mut file = File::open("resources/levels.json").map_err(|e| format!("file not found\n {}",e))?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| format!("file could not be read\n{}",e))?; 
            let level: Vec<Level> = serde_json::from_slice(&buffer[..]).map_err(|e| format!("file not valid\n{}",e))?;
            Ok(level)
        }
       let result = load_helper();
        match result {
            Ok(levels) => levels,
            Err(msg) => { // If we get an error, load the default and save a new levels.json file
                println!("Error loading level file.\nUsing default\n{}",msg);
                let level = Level::new();
                let serialized = serde_json::to_string_pretty(&level).unwrap();
                let mut file = File::create("resources/levels.json").unwrap();;
                file.write_all(serialized.as_bytes()).unwrap();
                level

            }
        }
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

                level
    }
}
