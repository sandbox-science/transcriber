pub mod audio;
pub mod cli;
pub mod subtitle;
pub mod types;

use crate::audio::Audio;
use crate::cli::Opts;
use crate::subtitle::SubtitleGenerator;
use crate::types::StyleConfig;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::fs::File;
use std::path::Path;

pub fn run_cli() -> Result<()> {
    env_logger::init();
    let opts: Opts = Opts::try_parse()?;

    let input_video = opts.input;
    let output_video = opts.output;

    let temp_dir = Path::new("build");
    std::fs::create_dir_all(temp_dir).context("Failed to create temp directory")?;

    let base = Path::new(&input_video)
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid input video path")?;

    let audio_path = temp_dir.join(format!("{}_audio.wav", base));
    let srt_path = temp_dir.join(format!("{}.ass", base));

    Audio::extract(&input_video, audio_path.to_str().unwrap())?;

    let segments = Audio::transcribe("model/ggml-tiny.bin", audio_path.to_str().unwrap())?;
    let style_config: StyleConfig = serde_json::from_reader(File::open("style.json")?)?;

    SubtitleGenerator::generate(segments, srt_path.to_str().unwrap(), &style_config)?;
    SubtitleGenerator::burn(
        &input_video,
        srt_path.to_str().unwrap(),
        &output_video,
        &style_config,
    )?;

    info!("Done! Video saved to: {}", output_video);
    Ok(())
}
