use crate::assets::*;
use crate::message::*;
use ggez::Context;

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::str;

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Deserialize, Serialize)]
pub struct Level {
    pub waves: Vec<Wave>,
    pub background_file: String,
    pub title: String,
    pub unlocked: [bool;4],
}

#[derive(Deserialize, Serialize)]
pub struct Wave {
    pub groups: Vec<WaveGroup>,
}

#[derive(Deserialize, Serialize)]
pub struct WaveGroup {
    pub operation: Operation,
    pub speed: f32,
    pub num_ships: usize,
    pub max_number: i32,
    pub min_number: i32,
}

pub const DIFFICULTY_NAMES: [&str;4] = ["Rookie","Cadet","Veteran","Space Marine"];
pub const SPEED_DIFFICULTY: [f32; 4] = [1.0, 1.1, 1.25, 1.5];
pub const MAX_NUMBER_DIFFICULTY: [f32; 4] = [1.0, 1.25, 2.0, 3.0];
pub const MIN_NUMBER_DIFFICULTY: [f32; 4] = [1.0, 1.25, 2.0, 3.0];
pub const NUM_SHIPS_DIFFICULTY: [f32; 4] = [1.0, 1.25, 2.0, 3.0];

impl Level {
    pub fn push_title(&self, messages: &mut VecDeque<Message>, assets: &Assets, ctx: &mut Context) {
        messages.push_back(Message::new(self.title.clone(), 2000.0, assets, ctx));
    }

    pub fn load_from_file() -> Vec<Level> {
        //if any of this fails, call new instead
        fn load_helper() -> Result<Vec<Level>, String> {
            let mut file = File::open("resources/levels.json")
                .map_err(|e| format!("file not found\n {}", e))?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| format!("file could not be read\n{}", e))?;
            let level: Vec<Level> = serde_json::from_slice(&buffer[..])
                .map_err(|e| format!("file not valid\n{}", e))?;
            Ok(level)
        }

        let result = load_helper();
        match result {
            Ok(levels) => levels,
            Err(msg) => {
                // If we get an error, load the default and save a new levels.json file
                println!("Error loading level file.\nUsing default\n{}", msg);
                let levels = Level::new();
                Level::save_levels(&levels);
                levels
            }
        }
    }

    pub fn save_levels(levels: &Vec<Level>) {
        let serialized = serde_json::to_string_pretty(&levels).unwrap(); // TODO do we want to handle this gracefully too?
        let mut file = File::create("resources/levels.json").unwrap();;
        file.write_all(serialized.as_bytes()).unwrap();
    }

    // consider validating max_number to be sure it makes sense
    pub fn new() -> Vec<Level> {
        vec![
            //Level 1
            Level {
                unlocked: [true,true,true,true],
                title: "Addition Attack!".to_string(),
                background_file: "/spacebg1.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 2.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Add,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 3.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Add,
                            num_ships: 8,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 4.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Add,
                            num_ships: 10,
                        }],
                    },
                ],
            },
            //Level 2
            Level {
                unlocked: [false,false,false,false],
                title: "Subtraction Subterfuge!".to_string(),
                background_file: "/spacebg2.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 2.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Subtract,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 3.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Subtract,
                            num_ships: 8,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 4.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Subtract,
                            num_ships: 10,
                        }],
                    },
                ],
            },
            //Level 3
            Level {
                unlocked: [false,false,false,false],
                title: "Multiplication Mayhem!".to_string(),
                background_file: "/spacebg3.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 2.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Multiply,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 3.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Multiply,
                            num_ships: 8,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 4.5,
                            max_number: 5,
                            min_number: 0,
                            operation: Operation::Multiply,
                            num_ships: 10,
                        }],
                    },
                ],
            },
            //Level 4
            Level {
                unlocked: [false,false,false,false],
                title: "Division Disaster!".to_string(),
                background_file: "/spacebg4.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 2.5,
                            max_number: 6,
                            min_number: 0,
                            operation: Operation::Divide,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 3.5,
                            max_number: 6,
                            min_number: 0,
                            operation: Operation::Divide,
                            num_ships: 8,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 4.5,
                            max_number: 6,
                            min_number: 0,
                            operation: Operation::Divide,
                            num_ships: 10,
                        }],
                    },
                ],
            },
            //Level 5
            Level {
                unlocked: [false,false,false,false],
                title: "The Final Assault!".to_string(),
                background_file: "/spacebg5.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![
                            WaveGroup {
                                speed: 3.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Add,
                                num_ships: 5,
                            },
                            WaveGroup {
                                speed: 2.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Subtract,
                                num_ships: 5,
                            },
                        ],
                    },
                    Wave {
                        groups: vec![
                            WaveGroup {
                                speed: 3.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Add,
                                num_ships: 3,
                            },
                            WaveGroup {
                                speed: 2.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Subtract,
                                num_ships: 3,
                            },
                            WaveGroup {
                                speed: 2.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Multiply,
                                num_ships: 3,
                            },
                        ],
                    },
                    Wave {
                        groups: vec![
                            WaveGroup {
                                speed: 3.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Add,
                                num_ships: 3,
                            },
                            WaveGroup {
                                speed: 3.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Subtract,
                                num_ships: 3,
                            },
                            WaveGroup {
                                speed: 2.5,
                                max_number: 5,
                                min_number: 0,
                                operation: Operation::Multiply,
                                num_ships: 3,
                            },
                            WaveGroup {
                                speed: 1.5,
                                max_number: 6,
                                min_number: 0,
                                operation: Operation::Divide,
                                num_ships: 3,
                            },
                        ],
                    },
                ],
            },
        ]
    }
}
