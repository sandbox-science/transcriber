# Transcriber

A fast, Rust video subtitle pipeline that:

- Extracts audio from a video using `ffmpeg`
- Transcribes it to text with timestamps using `whisper` (currently using Python for this, need to convert using Rust native)
- Generates ASS subtitle files with styling
- Burns subtitles into the original video with `ffmpeg`

> [!IMPORTANT]
>
> This project has been tested on macOS only. Thus, the following information assumes your system is a Mac.

## Engineering Requirements Document
The Engineering Requirements Document (**ERD**) is available here : https://docs.google.com/document/d/1xfLcfE5BA1i_wjUSHHJYuA4zneVqrm4rEg2bf_YeltQ/edit?usp=sharing

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

---

> [!NOTE]
>
> **TODO:** Currently, I am using a Python script to load **openai-whisper**. We need to implement a native Rust solution instead [Ticket Open](https://github.com/sandbox-science/transcriber/issues/1).