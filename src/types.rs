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

#[derive(Debug, Deserialize)]
pub struct BackgroundConfig {
    pub r#type: String,
    pub solid_color: Option<String>,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StyleConfig {
    pub text_alignment: String,
    pub vertical_position: String,
    pub font_family: String,
    pub font_size_px: u32,
    pub text_color: String,
    pub highlight_color: String,
    pub background: BackgroundConfig,
}
