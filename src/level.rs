
#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct LevelSpec {
    pub speed: f32,
    pub max_number: i32,
    pub operations: Vec<Operation>,
    pub num_ships: usize,
}
