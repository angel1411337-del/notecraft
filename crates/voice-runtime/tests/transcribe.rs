use craft_voice_runtime::audio::PcmAudio;
use craft_voice_runtime::transcribe::parse_text;
use craft_voice_runtime::{Cancellation, TranscribeConfig};

#[test]
fn the_transcript_file_is_words_with_silence_labels_dropped() {
    assert_eq!(parse_text(b"Call Ana tomorrow.\n").expect("text"), "Call Ana tomorrow.");
    assert_eq!(parse_text(b"First line\n[BLANK_AUDIO]\nsecond line\n").expect("text"), "First line second line");
    assert_eq!(parse_text(b"[BLANK_AUDIO]\n").expect("silence"), "");
    assert_eq!(parse_text(b"(double-check the date)\n").expect("aside"), "(double-check the date)");
    assert!(parse_text(&[0xff, 0xfe]).is_err());
    assert!(parse_text("x".repeat(65537).as_bytes()).is_err());
}

#[test]
fn a_missing_program_or_a_model_that_is_not_gguf_is_a_setup_error() {
    let t = tempfile::tempdir().expect("temporary root");
    let model = t.path().join("model.bin");
    std::fs::write(&model, b"lmgg not a gguf file").expect("model");
    let program = t.path().join("transcribe-cli");
    std::fs::write(&program, b"").expect("program");
    let missing = TranscribeConfig::new(t.path().join("missing"), model.clone(), t.path().into());
    assert!(missing.validate(&Cancellation::default()).unwrap_err().contains("transcribe-cli"));
    let not_gguf = TranscribeConfig::new(program, model, t.path().into());
    assert!(not_gguf.validate(&Cancellation::default()).unwrap_err().contains("GGUF"));
}

#[cfg(unix)]
mod process {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    /// A stand-in transcribe-cli: prints the options for `--help`, records its arguments, and
    /// writes `words` to the `-o` file, or exits with `status` without writing.
    fn stand_in(dir: &Path, words: &str, status: i32) -> PathBuf {
        let program = dir.join("transcribe-cli");
        let script = format!(
            "#!/bin/sh\n\
             if [ \"$1\" = --help ]; then echo '  -m, --model PATH'; echo '  -l, --language ISO'; \
             echo '  -q, --quiet'; echo '  -o, --output PATH'; exit 0; fi\n\
             printf '%s\\n' \"$@\" > \"{args}\"\n\
             [ {status} -ne 0 ] && exit {status}\n\
             while [ $# -gt 0 ]; do if [ \"$1\" = -o ]; then printf '%s\\n' \"{words}\" > \"$2\"; fi; shift; done\n",
            args = dir.join("args").display(),
        );
        std::fs::write(&program, script).expect("script");
        std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("executable");
        program
    }

    fn gguf(dir: &Path) -> PathBuf {
        let model = dir.join("model.gguf");
        std::fs::write(&model, b"GGUF\x03\x00\x00\x00").expect("model");
        model
    }

    fn speech() -> PcmAudio {
        PcmAudio::new(vec![1000; craft_voice_runtime::audio::SAMPLE_RATE * 2]).expect("audio")
    }

    #[test]
    fn runs_the_model_quietly_reads_only_the_output_file_and_passes_the_language() {
        let t = tempfile::tempdir().expect("dir");
        let mut config = TranscribeConfig::new(stand_in(t.path(), "Hola mundo.", 0), gguf(t.path()), t.path().join("jobs"));
        config.language = "es".into();
        config.validate(&Cancellation::default()).expect("valid");
        assert_eq!(config.recognize(&speech(), &Cancellation::default()).expect("words"), "Hola mundo.");
        let args: Vec<String> = std::fs::read_to_string(t.path().join("args")).expect("args").lines().map(String::from).collect();
        assert_eq!(args[..4], ["-q", "-m", t.path().join("model.gguf").to_str().expect("utf-8"), "-o"]);
        assert!(args.windows(2).any(|w| w[0] == "-l" && w[1] == "es"));
        assert!(args.last().expect("audio").ends_with("input.wav"));
    }

    #[test]
    fn automatic_language_passes_no_hint_and_job_files_are_removed() {
        let t = tempfile::tempdir().expect("dir");
        let config = TranscribeConfig::new(stand_in(t.path(), "Hello.", 0), gguf(t.path()), t.path().join("jobs"));
        assert_eq!(config.recognize(&speech(), &Cancellation::default()).expect("words"), "Hello.");
        let args = std::fs::read_to_string(t.path().join("args")).expect("args");
        assert!(!args.lines().any(|a| a == "-l"));
        let left: Vec<_> = std::fs::read_dir(t.path().join("jobs")).expect("jobs").flatten().collect();
        assert!(left.is_empty(), "no audio or transcript is left behind: {left:?}");
    }

    #[test]
    fn a_failed_run_is_an_error_not_an_empty_transcript() {
        let t = tempfile::tempdir().expect("dir");
        let config = TranscribeConfig::new(stand_in(t.path(), "unused", 3), gguf(t.path()), t.path().join("jobs"));
        assert!(config.recognize(&speech(), &Cancellation::default()).unwrap_err().contains("recognizer exited"));
    }

    #[test]
    fn a_program_without_the_needed_options_is_refused() {
        let t = tempfile::tempdir().expect("dir");
        let program = t.path().join("other-cli");
        std::fs::write(&program, "#!/bin/sh\necho 'usage: other [-x]'\n").expect("script");
        std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("executable");
        let config = TranscribeConfig::new(program, gguf(t.path()), t.path().join("jobs"));
        assert!(config.validate(&Cancellation::default()).unwrap_err().contains("does not advertise"));
    }

    #[test]
    fn a_cancelled_recording_never_starts_the_program() {
        let t = tempfile::tempdir().expect("dir");
        let config = TranscribeConfig::new(stand_in(t.path(), "unused", 0), gguf(t.path()), t.path().join("jobs"));
        let cancel = Cancellation::default();
        cancel.cancel();
        assert!(config.recognize(&speech(), &cancel).is_err());
        assert!(!t.path().join("args").exists());
    }
}
