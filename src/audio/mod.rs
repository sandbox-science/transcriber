mod extractor;
mod whisper;

pub use extractor::FfmpegExtractor;
pub use whisper::WhisperCapski;

use crate::types::Segment;
use anyhow::Result;

pub trait Capski {
    fn transcribe(
        model_path: &str,
        audio_path: &str,
        translate: bool,
        language: &Option<String>,
    ) -> Result<Vec<Segment>>;
}

pub trait Extractor {
    fn extract(input: &str, output: &str) -> Result<()>;
}
