//! A standalone library consumer. Supply your own recognizer program (whisper.cpp's whisper-cli,
//! or transcribe.cpp's transcribe-cli for GGUF models), a model and a PCM16 WAV.
use craft_voice_runtime::{
    Runtime, RuntimeEvent, TranscribeConfig, Transcriber, WhisperConfig,
    audio::{MAX_SAMPLES, PcmAudio},
};
use std::{io::Read, path::PathBuf, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        return Err("usage: transcribe WHISPER_CLI|TRANSCRIBE_CLI MODEL WAV PRIVATE_TEMP_ROOT".into());
    }
    let file = std::fs::File::open(&args[2])?;
    let mut bytes = Vec::new();
    file.take((MAX_SAMPLES * 2 + 65537) as u64).read_to_end(&mut bytes)?;
    let audio = PcmAudio::read_wav(&bytes)?;
    let (program, model, temporary) = (PathBuf::from(&args[0]), PathBuf::from(&args[1]), PathBuf::from(&args[3]));
    let transcribe_cpp = program.file_stem().is_some_and(|stem| stem == "transcribe-cli");
    let config: Box<dyn Transcriber> = if transcribe_cpp {
        Box::new(TranscribeConfig::new(program, model, temporary))
    } else {
        Box::new(WhisperConfig::new(program, model, temporary))
    };
    let mut runtime = Runtime::from_audio(1, config, audio)?;
    loop {
        for event in runtime.poll() {
            match event {
                RuntimeEvent::Transcript(t) if t.is_final => {
                    println!("{}", t.text);
                    return Ok(());
                }
                RuntimeEvent::Error { message, .. } => return Err(message.into()),
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
