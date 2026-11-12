//! A user-selected transcribe.cpp `transcribe-cli` process, which runs GGUF speech models such as
//! Parakeet, Canary, Granite Speech, Cohere Transcribe, Qwen3-ASR, Voxtral and Whisper. No shell,
//! downloads, or provider accounts.

use crate::whisper::{MIN_SAMPLES, advertised, non_speech, read_bounded, suppress_console, wait_child};
use crate::{Cancellation, Transcriber, audio::PcmAudio, temporary::JobFiles};
use craft_voice_core::{Capabilities, MAX_TRANSCRIPT_BYTES};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};

/// Executables that passed the `--help` check, by path, size and modification time.
type Checked = HashMap<PathBuf, (u64, Option<SystemTime>)>;
static CHECKED: OnceLock<Mutex<Checked>> = OnceLock::new();

/// The options this adapter passes. `-o` writes only the transcript to a file; standard output
/// carries diagnostics, which are never read.
const REQUIRED: [&str; 4] = ["-m", "-o", "-q", "-l"];

#[derive(Clone)]
pub struct TranscribeConfig {
    pub executable: PathBuf,
    pub model: PathBuf,
    /// `auto`, or the language code passed as the model's hint.
    pub language: String,
    pub temporary_root: PathBuf,
    pub timeout: Duration,
}

impl TranscribeConfig {
    pub fn new(executable: PathBuf, model: PathBuf, temporary_root: PathBuf) -> Self {
        Self { executable, model, temporary_root, language: "auto".into(), timeout: Duration::from_secs(120) }
    }

    pub fn validate(&self, cancel: &Cancellation) -> Result<(), String> {
        if !self.executable.is_absolute() || !self.executable.is_file() {
            return Err("choose an existing absolute path to transcribe-cli".into());
        }
        if !self.model.is_absolute() || !is_gguf(&self.model) {
            return Err("choose an existing local GGUF model file for transcribe-cli".into());
        }
        if self.language.len() > 16 || self.language.is_empty() || !self.language.chars().all(|c| c.is_ascii_alphabetic() || c == '-') {
            return Err("choose auto or a language code such as en or es".into());
        }
        let id = self.executable.metadata().ok().map(|m| (m.len(), m.modified().ok()));
        let cache = CHECKED.get_or_init(Default::default);
        if let (Some(id), Ok(known)) = (id, cache.lock())
            && known.get(&self.executable) == Some(&id)
        {
            return Ok(());
        }
        let files = JobFiles::new(&self.temporary_root)?;
        let help_path = files.dir.path().join("help");
        let out = File::create(&help_path).map_err(|e| e.to_string())?;
        let err = out.try_clone().map_err(|e| e.to_string())?;
        let mut command = Command::new(&self.executable);
        command.arg("--help").stdin(Stdio::null()).stdout(out).stderr(err);
        suppress_console(&mut command);
        let mut child = command.spawn().map_err(|e| format!("cannot start transcribe-cli: {e}"))?;
        let result = wait_child(&mut child, cancel, Duration::from_secs(10));
        if cancel.is_cancelled() {
            return Err("voice operation cancelled".into());
        }
        if result.as_ref().is_err_and(|e| e == "recognizer timed out") {
            return result;
        }
        let bytes = read_bounded(&help_path)?;
        let help = String::from_utf8_lossy(&bytes);
        let flags = advertised(&help);
        if !REQUIRED.iter().all(|flag| flags.contains(flag)) {
            return Err("this transcribe-cli does not advertise the required model/output/quiet/language options".into());
        }
        if let (Some(id), Ok(mut known)) = (id, cache.lock()) {
            if known.len() >= 32 {
                known.clear();
            }
            known.insert(self.executable.clone(), id);
        }
        Ok(())
    }

    pub fn recognize(&self, audio: &PcmAudio, cancel: &Cancellation) -> Result<String, String> {
        if audio.samples().is_empty() {
            return Ok(String::new());
        }
        if cancel.is_cancelled() {
            return Err("voice operation cancelled".into());
        }
        let files = JobFiles::new(&self.temporary_root)?;
        let wav = files.dir.path().join("input.wav");
        let output = files.dir.path().join("result.txt");
        // Whisper models skip input shorter than a second; the others take the padding as silence.
        let bytes = if audio.samples().len() < MIN_SAMPLES {
            let mut samples = audio.samples().to_vec();
            samples.resize(MIN_SAMPLES, 0);
            PcmAudio::new(samples)?.wav()
        } else {
            audio.wav()
        };
        fs::write(&wav, bytes).map_err(|e| format!("cannot stage voice audio: {e}"))?;
        let mut command = Command::new(&self.executable);
        command.arg("-q").arg("-m").arg(&self.model).arg("-o").arg(&output);
        if self.language != "auto" {
            command.arg("-l").arg(&self.language);
        }
        command.arg(&wav).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        suppress_console(&mut command);
        let mut child = command.spawn().map_err(|e| format!("cannot start transcribe-cli: {e}"))?;
        wait_child(&mut child, cancel, self.timeout)?;
        parse_text(&read_bounded(&output)?)
    }
}

impl Transcriber for TranscribeConfig {
    fn capabilities(&self) -> Capabilities {
        Capabilities { batch_transcription: true, partial_transcripts: true, language_selection: true, ..Capabilities::default() }
    }
    fn prepare(&mut self, cancel: &Cancellation) -> Result<(), String> {
        self.validate(cancel)
    }
    fn transcribe(&mut self, audio: &PcmAudio, cancel: &Cancellation) -> Result<String, String> {
        self.recognize(audio, cancel)
    }
}

/// A GGUF file starts with the bytes `GGUF`. Checking them tells a whisper.cpp `.bin` model,
/// or any other file, from one transcribe-cli can load, before a recording is spent on it.
fn is_gguf(path: &Path) -> bool {
    let mut magic = [0u8; 4];
    File::open(path).and_then(|mut f| f.read_exact(&mut magic)).is_ok() && &magic == b"GGUF"
}

/// The transcript file `-o` writes: the words, possibly over several lines. Lines that are only
/// a silence label (`[BLANK_AUDIO]`, which Whisper models emit) are dropped like whisper.cpp's.
pub fn parse_text(bytes: &[u8]) -> Result<String, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "recognizer returned text that is not UTF-8")?;
    let words: Vec<&str> = text.lines().map(str::trim).filter(|line| !non_speech(line)).collect();
    let joined = words.join(" ");
    if joined.len() > MAX_TRANSCRIPT_BYTES {
        return Err("transcript exceeds 64 KiB".into());
    }
    Ok(joined)
}
