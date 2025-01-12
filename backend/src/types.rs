use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Dimension {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize)]
pub struct Hint {
    pub x_hints: Vec<Vec<u32>>,
    pub y_hints: Vec<Vec<u32>>,
}

