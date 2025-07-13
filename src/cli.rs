use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Chris Dedman")]
pub struct Opts {
    #[arg(short, long, required = true)]
    pub input: String,

    #[arg(short, long, default_value = "output.mp4")]
    pub output: String,
}
