use capski::CapskiApp;
use capski::cli::Opts;

use anyhow::Result;
use clap::Parser;
use std::fs::File;

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();
    let style = serde_json::from_reader(File::open("style.json")?)?;

    let app = CapskiApp {
        input: opts.input,
        output: opts.output,
        model_path: "model/ggml-tiny.bin".to_string(),
        style,
        translate: opts.translate,
        language: Some(opts.language),
    };

    app.run()
}
