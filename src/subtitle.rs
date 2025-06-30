use crate::types::{Segment, StyleConfig, WordInfo};

use std::fs::File;
use std::io::Write;
use std::process::Command;
use anyhow::{Context, Result, anyhow};
use log::info;
pub struct SubtitleGenerator;

impl SubtitleGenerator {
    fn css_hex_to_ass(hex: &str) -> String {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return "&H00FFFFFF".to_string(); // default white
        }

        let r = &hex[0..2];
        let g = &hex[2..4];
        let b = &hex[4..6];

        format!("&H00{}{}{}", b, g, r)
    }

    fn map_alignment(horizontal: &str, vertical: &str) -> u8 {
        match (horizontal, vertical) {
            ("left", "bottom")   => 1,
            ("center", "bottom") => 2,
            ("right", "bottom")  => 3,
            ("left", "middle")   => 4,
            ("center", "middle") => 5,
            ("right", "middle")  => 6,
            ("left", "top")      => 7,
            ("center", "top")    => 8,
            ("right", "top")     => 9,
            _ => 2, // default to center bottom
        }
    }

    fn seconds_to_ass_time(secs: f32) -> String {
        let hours   = (secs / 3600.0) as u32;
        let minutes = ((secs % 3600.0) / 60.0) as u32;
        let seconds = (secs % 60.0) as f32;

        format!("{:01}:{:02}:{:05.2}", hours, minutes, seconds)
    }

    pub fn generate(segments: Vec<Segment>, ass_path: &str, style: &StyleConfig) -> Result<()> {
        let primary_color   = Self::css_hex_to_ass(&style.text_color);
        let highlight_color = Self::css_hex_to_ass(&style.highlight_color);
        let alignment       = Self::map_alignment(&style.text_alignment, &style.vertical_position);
        let border_style    = style.border_style.unwrap_or(1);

        let back_color = if border_style == 3 {
            Self::css_hex_to_ass(&style.highlight_color)
        } else {
            "&H00000000".to_string() // if none set, apply transparent background
        };

        let bold = match style.font_weight.as_str() {
            "bold" => -1,
            _ => 0,
        };

        let mut ass_content = String::new();
        ass_content.push_str("[Script Info]\n");
        ass_content.push_str("Title: Styled Subs\n");
        ass_content.push_str("ScriptType: v4.00+\n");
        ass_content.push_str("PlayResY: 720\n");
        ass_content.push_str("PlayResX: 1280\n\n");

        ass_content.push_str("[V4+ Styles]\n");
        ass_content.push_str("Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n");
        ass_content.push_str(&format!(
            "Style: Default,{},{},{},{},{},{},{},0,0,0,100,100,0,0,{},2,2,{},10,10,10,1\n\n",
            style.font_family,
            style.font_size_px,
            primary_color,
            highlight_color,
            style.outline_color,
            back_color,
            bold,
            border_style,
            alignment
        ));

        ass_content.push_str("[Events]\n");
        ass_content.push_str("Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n");

        // Here, we extract the words to render styling
        let mut words: Vec<WordInfo> = Vec::new();
        for seg in segments {
            for word in seg.words {
                words.push(word);
            }
        }

        // Render 3 words per sequence with the current spoken
        // word being highlighted.
        for i in 0..words.len() {
            let prev = if i >= 1 { &words[i-1].word } else { "" };
            let curr = &words[i].word;
            let next = if i+1 < words.len() { &words[i+1].word } else { "" };

            let start = words[i].start;
            let end   = words[i].end;

            ass_content.push_str(&format!(
                "Dialogue: 0,{}, {}, Default,,0,0,0,,{{\\an{}}}{{\\bord0}}{} {{\\bord3\\c&H00FFAA33&}}{} {{\\bord0\\c&H00FFFFFF&}}{}\n",
                Self::seconds_to_ass_time(start),
                Self::seconds_to_ass_time(end),
                alignment,
                prev, curr, next
            ));
        }

        File::create(ass_path)?.write_all(ass_content.as_bytes())?;
        Ok(())
    }


    pub fn burn(audio_path: &str, ass_path: &str, output_path: &str, style: &StyleConfig) -> Result<()> {
        info!("Burning subtitles onto synthetic background...");

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y");

        match style.background.r#type.as_str() {
            "solid" => {
                let color = style.background.solid_color.as_deref().unwrap_or("#000000");
                let color_hex = &color[1..];
                cmd.args(&["-f", "lavfi", "-i", &format!("color=c=0x{}:s=1280x720:d=3600", color_hex)]);
            },
            "image" => {
                let img = style.background.image_url.as_deref().expect("imageUrl required for image background");
                cmd.args(&["-loop", "1", "-i", img]);
            },
            "video" => {
                let vid = style.background.video_url.as_deref().expect("videoUrl required for video background");
                cmd.args(&["-i", vid]);
            },
            _ => {
                // fallback
                cmd.args(&["-f", "lavfi", "-i", "color=c=black:s=1280x720:d=3600"]);
            }
        }

        cmd.args(&["-i", audio_path, "-vf", &format!("ass={}", ass_path), "-c:a", "copy", "-shortest", output_path]);

        let status = cmd.status().context("Failed to run ffmpeg")?;
        if !status.success() {
            return Err(anyhow!("FFmpeg failed to render video {}", output_path));
        }
        Ok(())
    }
}