use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Capski",
    version = "0.1.0",
    author = "Chris Dedman",
    about = "Create karaoke-style videos from audio or video",
    disable_help_flag = false,
    disable_version_flag = false
)]
pub struct Opts {
    #[arg(short, long, required = true)]
    pub input: String,

    #[arg(short, long, default_value = "output.mp4")]
    pub output: String,
}
