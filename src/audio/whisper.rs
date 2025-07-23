use crate::types::Segment;

use super::Capski;
use anyhow::{Context, Result};
use log::info;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperCapski;

impl Capski for WhisperCapski {
    // Function to transcribe audio using the Whisper model
    fn transcribe(model_path: &str, audio_path: &str) -> Result<Vec<Segment>> {
        info!("Transcribing with Whisper...");

        let reader = hound::WavReader::open(audio_path)
            .with_context(|| format!("failed to open audio file: {}", audio_path))?;

        // Read WAV file and collect samples
        let samples: Vec<i16> = reader.into_samples::<i16>().map(|x| x.unwrap()).collect();

        // Load the Whisper model
        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())?;
        let mut state = ctx.create_state()?;

        // Set up parameters for the Whisper model
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
        params.set_translate(false);
        params.set_language(Some("auto"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_token_timestamps(true);

        let mut audio = vec![0.0f32; samples.len()];
        whisper_rs::convert_integer_to_float_audio(&samples, &mut audio)?;

        // Run the model
        state
            .full(params, &audio[..])
            .context("Failed to run Whisper model")?;

        let mut segments: Vec<Segment> = Vec::new();
        let num_segments = state.full_n_segments()?;
        for i in 0..num_segments {
            let num_tokens = state.full_n_tokens(i)?;
            if num_tokens == 0 {
                continue;
            }

            // start with a fresh word buffer for this segment
            let mut current_word = String::new();
            let mut start_time = 0.0;
            let mut end_time = 0.0;

            for token_i in 0..num_tokens {
                let token_data = state.full_get_token_data(i, token_i)?;
                let segment_text = state.full_get_token_text(i, token_i)?;

                if segment_text.starts_with("[_") && segment_text.ends_with("]") {
                    continue;
                }

                if segment_text.starts_with(" ") {
                    // here, we flush the previous word
                    if !current_word.is_empty() {
                        segments.push(Segment {
                            start: start_time,
                            end: end_time,
                            word: current_word.trim().to_string(),
                        });
                    }
                    // then, start a new word
                    current_word = segment_text.trim_start().to_string();
                    start_time = token_data.t0 as f32 / 100.0;
                    end_time = token_data.t1 as f32 / 100.0;
                } else {
                    current_word.push_str(&segment_text);
                    end_time = token_data.t1 as f32 / 100.0;
                }
            }

            // finally, flush the last word of this segment
            if !current_word.is_empty() {
                segments.push(Segment {
                    start: start_time,
                    end: end_time,
                    word: current_word.trim().to_string(),
                });
            }
        }

        Ok(segments)
    }
}
