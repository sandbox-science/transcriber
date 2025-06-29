# Transcriber

A fast, Rust video subtitle pipeline that:

- Extracts audio from a video using `ffmpeg`
- Transcribes it to text with timestamps using `whisper` (currently using Python for this, need to convert using Rust native)
- Generates ASS subtitle files with styling
- Burns subtitles into the original video with `ffmpeg`

> [!IMPORTANT]
>
> This project has been tested on macOS only. Thus, the following information assumes your system is a Mac.

---

> [!NOTE]
>
> **TODO:** Currently, I am using a Python script to load **openai-whisper**. We need to implement a native Rust solution instead.

---

## Getting started

### 0. Install ffmpeg

```bash
brew install ffmpeg
```

### 1. Load python virtual env
This is needed because I am currently loading Whisper through Python.
```bash
python -m venv .venv
source .venv/bin/activate
pip3 install openai-whisper
```

### 2. Build Transcriber
You can now build the transcriber using Rust Cargo.
```bash
cargo build
```

### 3. Run Transcriber
```bash
cargo run -- --input "test2.mov"
```

