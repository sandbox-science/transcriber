use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Capski",
    version = "0.2.0",
    author = "Chris Dedman",
    about = "Create karaoke-style videos from audio or video.",
    disable_help_flag = false,
    disable_version_flag = false
)]
pub struct Opts {
    #[arg(short, long, required = true)]
    pub input: String,

    #[arg(short, long, default_value = "output.mp4")]
    pub output: String,

    #[arg(
        long,
        default_value_t = false,
        help = "Translate from the source language to English."
    )]
    pub translate: bool,

    #[arg(
        long,
        default_value = "auto",
        help = "Specify the source language ('fr', 'es', etc). Defaults to 'auto'."
    )]
    pub language: String,
}
