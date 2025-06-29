mod cli;
mod audio;
mod subtitle;
mod types;

use crate::types::StyleConfig;

use cli::Opts;
use audio::Audio;
use subtitle::SubtitleGenerator;

use anyhow::{Result, Context};
use log::info;
use std::path::Path;
use std::fs::File;
use clap::Parser;

fn main() -> Result<()> {
    env_logger::init();
    let opts: Opts = Opts::parse();

    let input_video  = opts.input;
    let output_video = opts.output;

    let temp_dir = Path::new("build");
    std::fs::create_dir_all(temp_dir).context("Failed to create temp directory")?;

    let base = Path::new(&input_video)
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid input video path")?;

    let audio_path = temp_dir.join(format!("{}_audio.wav", base));
    let srt_path   = temp_dir.join(format!("{}.ass", base));

    Audio::extract(&input_video, audio_path.to_str().unwrap())?;

    let segments = Audio::transcribe(audio_path.to_str().unwrap())?;
    let style_config: StyleConfig = serde_json::from_reader(File::open("style.json")?)?;

    SubtitleGenerator::generate(segments, srt_path.to_str().unwrap(), &style_config)?;
    SubtitleGenerator::burn(&input_video, srt_path.to_str().unwrap(), &output_video, &style_config)?;

    info!("Done! Video saved to: {}", output_video);
    Ok(())
}
