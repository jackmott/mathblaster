use crate::message::*;
use crate::assets::*;

use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Level {
    pub waves: Vec<Wave>,
    pub background_file: String,
    pub title: String,
}
pub struct Wave {
    pub groups: Vec<WaveGroup>,
}
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

    pub fn new() -> Vec<Level> {
        vec![
            //Level 1
            Level {
                title:"Addition Attack!".to_string(),
                background_file: "/spacebg1.jpg".to_string(),
                waves: vec![
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 6.5,
                            max_number: 5,
                            operation: Operation::Add,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 6.5,
                            max_number: 5,
                            operation: Operation::Add,
                            num_ships: 5,
                        }],
                    },
                    Wave {
                        groups: vec![WaveGroup {
                            speed: 6.5,
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
        ]
    }
}
