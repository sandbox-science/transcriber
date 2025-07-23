use super::Extractor;
use anyhow::{Context, Result};
use log::info;
use std::process::Command;

pub struct FfmpegExtractor;

impl Extractor for FfmpegExtractor {
    // Function to extract audio from a video file
    fn extract(input: &str, output: &str) -> Result<()> {
        info!("Extracting audio from {} to {}", input, output);

        let status = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-i",
                input,
                "-vn",
                "-acodec",
                "pcm_s16le",
                "-ar",
                "16000",
                "-ac",
                "1",
                output,
            ])
            .status()
            .context("Failed to extract audio via FFmpeg")?;

        if !status.success() {
            anyhow::bail!(
                "ffmpeg failed to extract audio: exited with code {}",
                status
            );
        }

        Ok(())
    }
}
