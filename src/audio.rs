use crate::types::Segment;

use std::process::Command;
use anyhow::{Context, Result};
use log::{info, error};

pub struct Audio;

impl Audio {
    // TODO: use Rust native whisper-rs instead of Python openai-whisper
    pub fn transcribe(audio_path: &str) -> Result<Vec<Segment>> {
        info!("Transcribing with Whisper...");

        let output = Command::new("python3")
            .arg("utils/transcribe.py")
            .arg(audio_path)
            .output()
            .context("Failed to execute Python script")?;

        if !output.status.success() {
            let error_message = String::from_utf8(output.stderr).context("Failed to read stderr")?;
            error!("Python script error: {}", error_message);
            return Err(anyhow::anyhow!("Python script error: {}", error_message));
        }

        let segments_json          = String::from_utf8(output.stdout).context("Failed to read stdout")?;
        let segments: Vec<Segment> = serde_json::from_str(&segments_json).context("Failed to parse JSON")?;

        Ok(segments)
    }

    pub fn extract(video_path: &str, audio_path: &str) -> Result<()> {
        info!("Extracting audio from {} to {}", video_path, audio_path);

        let status = Command::new("ffmpeg")
            .args(&[
                "-y", "-i", video_path,
                "-vn", "-acodec", "pcm_s16le",
                "-ar", "16000", "-ac", "1", audio_path
            ])
            .status()
            .context("Failed to extract audio")?;

        if !status.success() {
            return Err(anyhow::anyhow!("ffmpeg failed to extract audio"));
        }

        Ok(())
    }
}