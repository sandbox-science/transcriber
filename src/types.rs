use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WordInfo {
    pub start: f32,
    pub end: f32,
    pub word: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub words: Vec<WordInfo>,
}
