# Transcriber

A fast, Rust video subtitle pipeline that:

- Extracts audio from a video using `ffmpeg`
- Transcribes speech into text with precise timestamps using `whisper_rs`
- Generates styled karaoke-style ASS subtitles (with word-level highlighting)
- Burns subtitles into videos or overlays on new video, image, or solid color backgrounds with `ffmpeg`

> [!NOTE]
> More examples are available in the [example](/example/) directory. The following is just a GIF for visual demo purposes

![DEMO](/example/gif_output_bg_image.gif)

> [!IMPORTANT]
>
> **Tested on macOS only.**
>
> The instructions below assume you’re on a Mac (using Homebrew).

## Engineering Requirements Document
The Engineering Requirements Document (**ERD**) is available here :

-> [ERD on Google Docs](https://docs.google.com/document/d/1xfLcfE5BA1i_wjUSHHJYuA4zneVqrm4rEg2bf_YeltQ/edit?usp=sharing)

## Getting started

### 1. Install ffmpeg
```bash
brew install ffmpeg
```

### 2. Build Transcriber
You can now build the transcriber using Rust Cargo.
```bash
cargo build
```

### 3. Run Transcriber
```bash
cargo run -- --input "example/input_audio.wav"
```

This runs the pipeline end-to-end:
- extracts or processes audio,
- transcribes with Whisper,
- generates subtitles,
- burns subtitles into a new video.

## Configuration
In the `style.json` file, you control:
- **Subtitle text styling**: font, size, colors, alignment
- **Background style**: solid color, image, or looping video

You can control your background type to be either `solid`, `video`, or `image`. Here are the 3 way to configure it:

- ### Video Background
```bash
{
  "background": {
    "type": "video",
    "solid_color": null,
    "video_path": "assets/bg_example.mp4",
    "image_path": null
  }
}
```
- ### Image Background
```bash
{
  "background": {
    "type": "image",
    "solid_color": null,
    "video_path": null,
    "image_path": "assets/bg_image.jpg"
  }
}
```
- ### Solid Background
```bash
{
  "background": {
    "type": "solid",
    "solid_color": "#101020",
    "video_path": null,
    "image_path": null
  }
}
```

> [!NOTE]
>
> You can find the example of each of those configurations in the [example](/example/) directory. In [assets](/assets/) directory, you will find the `video` and `image` backgrounds used in those examples.
