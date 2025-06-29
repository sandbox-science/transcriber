import whisper
import json
import sys


def transcribe_audio(audio_path):
    model = whisper.load_model("base")
    result = model.transcribe(audio_path, word_timestamps=True)
    return result["segments"]


if __name__ == "__main__":
    audio_path = sys.argv[1]
    segments = transcribe_audio(audio_path)
    print(json.dumps(segments))
