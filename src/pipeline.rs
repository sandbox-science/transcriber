use crate::audio::{Capski, Extractor, FfmpegExtractor, WhisperCapski};
use crate::subtitle::SubtitleGenerator;
use crate::types::StyleConfig;

use anyhow::{Context, Result};
use log::info;
use std::path::Path;

pub struct CapskiApp {
    pub input: String,
    pub output: String,
    pub translate: bool,
    pub language: Option<String>,
    pub model_path: String,
    pub style: StyleConfig,
}

impl CapskiApp {
    pub fn run(&self) -> Result<()> {
        let base = Path::new(&self.input)
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid input video path")?;

        let build_dir = Path::new("build");
        std::fs::create_dir_all(build_dir).context("Failed to create temp directory")?;

        let audio_path = build_dir.join(format!("{}_audio.wav", base));
        let subtitle_path = build_dir.join(format!("{}.ass", base));

        FfmpegExtractor::extract(&self.input, audio_path.to_str().unwrap())?;
        let segments = WhisperCapski::transcribe(
            &self.model_path,
            audio_path.to_str().unwrap(),
            self.translate,
            &self.language,
        )?;

        SubtitleGenerator::generate(segments, subtitle_path.to_str().unwrap(), &self.style)?;
        SubtitleGenerator::burn(
            &self.input,
            subtitle_path.to_str().unwrap(),
            &self.output,
            &self.style,
        )?;

        info!("Done! Video saved to: {}", self.output);
        Ok(())
    }
}
