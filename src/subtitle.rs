use crate::types::Segment;

use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use anyhow::{Context, Result, anyhow};
use log::info;


pub struct SubtitleGenerator;

impl SubtitleGenerator {
    pub fn generate(segments: Vec<Segment>, srt_path: &str) -> Result<()> {
        info!("Generating SRT...");

        let mut srt_content = String::new();
        let mut index       = 1;

        for seg in segments {
            for word_info in seg.words {
                let start   = Duration::from_secs_f32(word_info.start);
                let end     = Duration::from_secs_f32(word_info.end);
                let content = word_info.word.trim();
                if content.is_empty() {
                    continue;
                }

                let start_time = format!(
                    "{:02}:{:02}:{:02},{:03}",
                    start.as_secs() / 3600,
                    (start.as_secs() % 3600) / 60,
                    start.as_secs() % 60,
                    start.subsec_millis()
                );

                let end_time = format!(
                    "{:02}:{:02}:{:02},{:03}",
                    end.as_secs() / 3600,
                    (end.as_secs() % 3600) / 60,
                    end.as_secs() % 60,
                    end.subsec_millis()
                );

                srt_content.push_str(&format!("{}\n{} --> {}\n{}\n\n", index, start_time, end_time, content));
                index += 1;
            }
        }
        File::create(srt_path)?.write_all(srt_content.as_bytes())?;

        Ok(())
    }

    pub fn burn(video_path: &str, srt_path: &str, output_path: &str) -> Result<()> {
        info!("Burning subtitles into video...");

        let status = Command::new("ffmpeg")
            .args(["-y", "-i", video_path, "-vf", &format!("subtitles={}", srt_path), 
               "-c:a", "copy", output_path])
            .status()
            .context("Failed to run ffmpeg for subtitle burning")?;

        if !status.success() {
            return Err(anyhow!("FFmpeg subtitles burning failed for file {}", output_path));
        }

        Ok(())
    }
}