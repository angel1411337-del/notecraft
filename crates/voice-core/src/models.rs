//! The speech models dictation offers, ranked by accuracy. Data only: nothing here downloads,
//! opens or runs a model. The user downloads the file and picks it; the host runs the program.
//!
//! Ranked by the average word error rate (WER, lower is better) on the Hugging Face Open ASR
//! Leaderboard, down to Whisper large-v3, keeping only models a local program can run on a
//! laptop. Sizes, languages and licenses come from the transcribe.cpp model catalog; the file
//! named is its Q4_K_M build. `docs/how-to/dictate.md` lists the sources.

/// The local program that runs a model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Engine {
    /// `whisper-cli` from whisper.cpp, with a ggml Whisper model.
    WhisperCpp,
    /// `transcribe-cli` from transcribe.cpp, with a GGUF model.
    TranscribeCpp,
}

impl Engine {
    /// The program's file name, without a platform suffix.
    pub fn program(self) -> &'static str {
        match self {
            Self::WhisperCpp => "whisper-cli",
            Self::TranscribeCpp => "transcribe-cli",
        }
    }

    /// Where the program can be downloaded.
    pub fn page(self) -> &'static str {
        match self {
            Self::WhisperCpp => "https://github.com/ggml-org/whisper.cpp#quick-start",
            Self::TranscribeCpp => "https://github.com/handy-computer/transcribe.cpp/releases",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Model {
    /// A stable identifier, saved in settings.
    pub id: &'static str,
    pub name: &'static str,
    pub maker: &'static str,
    pub engine: Engine,
    /// The Open ASR Leaderboard average WER in percent, where the model is ranked there.
    pub leaderboard_wer: Option<f32>,
    /// The size of the suggested file, in megabytes.
    pub download_mb: u32,
    pub license: &'static str,
    /// ISO 639 language codes the model transcribes. Empty: whatever model file is chosen.
    pub languages: &'static [&'static str],
    /// Where the model's files are published.
    pub page: &'static str,
    /// The file to download from `page`. Empty when any file of the engine's format will do.
    pub suggested_file: &'static str,
}

impl Model {
    /// Whether the model transcribes `language` (`auto` always qualifies).
    pub fn speaks(&self, language: &str) -> bool {
        language == "auto" || self.languages.is_empty() || self.languages.contains(&language)
    }
}

/// Any whisper.cpp model the user already has: dictation's first setup.
pub const WHISPER_CPP: &str = "whisper-cpp";

/// The default for a new setup: the most accurate model that fits a modest laptop quickly.
pub const DEFAULT_MODEL: &str = "parakeet-tdt-0.6b-v3";

/// Every model offered, most accurate first, then the smaller Whisper models and whisper.cpp.
pub fn catalog() -> &'static [Model] {
    CATALOG
}

/// The model with this identifier.
pub fn model(id: &str) -> Option<&'static Model> {
    CATALOG.iter().find(|m| m.id == id)
}

const CATALOG: &[Model] = &[
    Model {
        id: "granite-speech-4.1-2b",
        name: "Granite Speech 4.1 2B",
        maker: "IBM",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(5.33),
        download_mb: 1603,
        license: "Apache-2.0",
        languages: &["en", "fr", "de", "es", "pt", "ja"],
        page: "https://huggingface.co/handy-computer/granite-speech-4.1-2b-gguf",
        suggested_file: "granite-speech-4.1-2b-Q4_K_M.gguf",
    },
    Model {
        id: "cohere-transcribe-03-2026",
        name: "Cohere Transcribe",
        maker: "Cohere",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(5.42),
        download_mb: 1558,
        license: "Apache-2.0",
        languages: &["en", "fr", "de", "es", "it", "pt", "nl", "pl", "el", "ar", "ja", "zh", "vi", "ko"],
        page: "https://huggingface.co/handy-computer/cohere-transcribe-03-2026-gguf",
        suggested_file: "cohere-transcribe-03-2026-Q4_K_M.gguf",
    },
    Model {
        id: "granite-4.0-1b-speech",
        name: "Granite 4.0 1B Speech",
        maker: "IBM",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(5.52),
        download_mb: 1603,
        license: "Apache-2.0",
        languages: &["en", "fr", "de", "es", "pt", "ja"],
        page: "https://huggingface.co/handy-computer/granite-4.0-1b-speech-gguf",
        suggested_file: "granite-4.0-1b-speech-Q4_K_M.gguf",
    },
    Model {
        id: "canary-qwen-2.5b",
        name: "Canary-Qwen 2.5B",
        maker: "NVIDIA",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(5.63),
        download_mb: 1738,
        license: "CC-BY-4.0",
        languages: &["en"],
        page: "https://huggingface.co/handy-computer/canary-qwen-2.5b-gguf",
        suggested_file: "canary-qwen-2.5b-Q4_K_M.gguf",
    },
    Model {
        id: "qwen3-asr-1.7b",
        name: "Qwen3-ASR 1.7B",
        maker: "Alibaba",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(5.76),
        download_mb: 1320,
        license: "Apache-2.0",
        languages: &[
            "zh", "en", "yue", "ar", "de", "fr", "es", "pt", "id", "it", "ko", "ru", "th", "vi", "ja", "tr", "hi", "ms", "nl", "sv", "da", "fi",
            "pl", "cs", "fil", "fa", "el", "ro", "hu", "mk",
        ],
        page: "https://huggingface.co/handy-computer/Qwen3-ASR-1.7B-gguf",
        suggested_file: "Qwen3-ASR-1.7B-Q4_K_M.gguf",
    },
    Model {
        id: "parakeet-unified-en-0.6b",
        name: "Parakeet Unified 0.6B",
        maker: "NVIDIA",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(5.91),
        download_mb: 477,
        license: "NVIDIA Open Model License",
        languages: &["en"],
        page: "https://huggingface.co/handy-computer/parakeet-unified-en-0.6b-gguf",
        suggested_file: "parakeet-unified-en-0.6b-Q4_K_M.gguf",
    },
    Model {
        id: "parakeet-tdt-0.6b-v2",
        name: "Parakeet TDT 0.6B v2",
        maker: "NVIDIA",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(6.05),
        download_mb: 475,
        license: "CC-BY-4.0",
        languages: &["en"],
        page: "https://huggingface.co/handy-computer/parakeet-tdt-0.6b-v2-gguf",
        suggested_file: "parakeet-tdt-0.6b-v2-Q4_K_M.gguf",
    },
    Model {
        id: "parakeet-tdt-0.6b-v3",
        name: "Parakeet TDT 0.6B v3",
        maker: "NVIDIA",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(6.34),
        download_mb: 485,
        license: "CC-BY-4.0",
        languages: &[
            "bg", "hr", "cs", "da", "nl", "en", "et", "fi", "fr", "de", "el", "hu", "it", "lv", "lt", "mt", "pl", "pt", "ro", "ru", "sk", "sl", "es",
            "sv", "uk",
        ],
        page: "https://huggingface.co/handy-computer/parakeet-tdt-0.6b-v3-gguf",
        suggested_file: "parakeet-tdt-0.6b-v3-Q4_K_M.gguf",
    },
    Model {
        id: "canary-1b",
        name: "Canary 1B",
        maker: "NVIDIA",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(6.50),
        download_mb: 730,
        license: "CC-BY-NC-4.0 (non-commercial)",
        languages: &["en", "de", "es", "fr"],
        page: "https://huggingface.co/handy-computer/canary-1b-gguf",
        suggested_file: "canary-1b-Q4_K_M.gguf",
    },
    Model {
        id: "nemotron-speech-streaming-en-0.6b",
        name: "Nemotron Speech Streaming 0.6B",
        maker: "NVIDIA",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(6.93),
        download_mb: 475,
        license: "NVIDIA Open Model License",
        languages: &["en"],
        page: "https://huggingface.co/handy-computer/nemotron-speech-streaming-en-0.6b-gguf",
        suggested_file: "nemotron-speech-streaming-en-0.6b-Q4_K_M.gguf",
    },
    Model {
        id: "parakeet-tdt-1.1b",
        name: "Parakeet TDT 1.1B",
        maker: "NVIDIA",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(7.02),
        download_mb: 825,
        license: "CC-BY-4.0",
        languages: &["en"],
        page: "https://huggingface.co/handy-computer/parakeet-tdt-1.1b-gguf",
        suggested_file: "parakeet-tdt-1.1b-Q4_K_M.gguf",
    },
    Model {
        id: "voxtral-mini-3b-2507",
        name: "Voxtral Mini 3B",
        maker: "Mistral AI",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(7.05),
        download_mb: 2985,
        license: "Apache-2.0",
        languages: &["en", "fr", "de", "es", "it", "pt", "nl", "hi"],
        page: "https://huggingface.co/handy-computer/Voxtral-Mini-3B-2507-gguf",
        suggested_file: "Voxtral-Mini-3B-2507-Q4_K_M.gguf",
    },
    Model {
        id: "whisper-large-v3",
        name: "Whisper large-v3",
        maker: "OpenAI",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(7.44),
        download_mb: 997,
        license: "Apache-2.0",
        languages: &[
            "af", "am", "ar", "as", "az", "ba", "be", "bg", "bn", "bo", "br", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa",
            "fi", "fo", "fr", "gl", "gu", "haw", "ha", "he", "hi", "hr", "ht", "hu", "hy", "id", "is", "it", "ja", "jw", "ka", "kk", "km", "kn",
            "ko", "la", "lb", "ln", "lo", "lt", "lv", "mg", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "ne", "nl", "nn", "no", "oc", "pa", "pl",
            "ps", "pt", "ro", "ru", "sa", "sd", "si", "sk", "sl", "sn", "so", "sq", "sr", "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr",
            "tt", "uk", "ur", "uz", "vi", "yi", "yo", "yue", "zh",
        ],
        page: "https://huggingface.co/handy-computer/whisper-large-v3-gguf",
        suggested_file: "whisper-large-v3-Q4_K_M.gguf",
    },
    Model {
        id: "whisper-large-v3-turbo",
        name: "Whisper large-v3 turbo",
        maker: "OpenAI",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: Some(7.83),
        download_mb: 536,
        license: "MIT",
        languages: &[
            "af", "am", "ar", "as", "az", "ba", "be", "bg", "bn", "bo", "br", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa",
            "fi", "fo", "fr", "gl", "gu", "haw", "ha", "he", "hi", "hr", "ht", "hu", "hy", "id", "is", "it", "ja", "jw", "ka", "kk", "km", "kn",
            "ko", "la", "lb", "ln", "lo", "lt", "lv", "mg", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "ne", "nl", "nn", "no", "oc", "pa", "pl",
            "ps", "pt", "ro", "ru", "sa", "sd", "si", "sk", "sl", "sn", "so", "sq", "sr", "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr",
            "tt", "uk", "ur", "uz", "vi", "yi", "yo", "yue", "zh",
        ],
        page: "https://huggingface.co/handy-computer/whisper-large-v3-turbo-gguf",
        suggested_file: "whisper-large-v3-turbo-Q4_K_M.gguf",
    },
    Model {
        id: "whisper-small",
        name: "Whisper small",
        maker: "OpenAI",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: None,
        download_mb: 172,
        license: "Apache-2.0",
        languages: &[
            "af", "am", "ar", "as", "az", "ba", "be", "bg", "bn", "bo", "br", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa",
            "fi", "fo", "fr", "gl", "gu", "haw", "ha", "he", "hi", "hr", "ht", "hu", "hy", "id", "is", "it", "ja", "jw", "ka", "kk", "km", "kn",
            "ko", "la", "lb", "ln", "lo", "lt", "lv", "mg", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "ne", "nl", "nn", "no", "oc", "pa", "pl",
            "ps", "pt", "ro", "ru", "sa", "sd", "si", "sk", "sl", "sn", "so", "sq", "sr", "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr",
            "tt", "uk", "ur", "uz", "vi", "yi", "yo", "zh",
        ],
        page: "https://huggingface.co/handy-computer/whisper-small-gguf",
        suggested_file: "whisper-small-Q4_K_M.gguf",
    },
    Model {
        id: "whisper-base",
        name: "Whisper base",
        maker: "OpenAI",
        engine: Engine::TranscribeCpp,
        leaderboard_wer: None,
        download_mb: 59,
        license: "Apache-2.0",
        languages: &[
            "af", "am", "ar", "as", "az", "ba", "be", "bg", "bn", "bo", "br", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa",
            "fi", "fo", "fr", "gl", "gu", "haw", "ha", "he", "hi", "hr", "ht", "hu", "hy", "id", "is", "it", "ja", "jw", "ka", "kk", "km", "kn",
            "ko", "la", "lb", "ln", "lo", "lt", "lv", "mg", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "ne", "nl", "nn", "no", "oc", "pa", "pl",
            "ps", "pt", "ro", "ru", "sa", "sd", "si", "sk", "sl", "sn", "so", "sq", "sr", "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr",
            "tt", "uk", "ur", "uz", "vi", "yi", "yo", "zh",
        ],
        page: "https://huggingface.co/handy-computer/whisper-base-gguf",
        suggested_file: "whisper-base-Q4_K_M.gguf",
    },
    WHISPER_CPP_MODEL,
];

/// Any whisper.cpp model file, run by `whisper-cli`: dictation's first setup. Last in the list.
pub const WHISPER_CPP_MODEL: Model = Model {
    id: WHISPER_CPP,
    name: "whisper.cpp model",
    maker: "OpenAI",
    engine: Engine::WhisperCpp,
    leaderboard_wer: None,
    download_mb: 0,
    license: "MIT",
    languages: &[],
    page: "https://huggingface.co/ggerganov/whisper.cpp/tree/main",
    suggested_file: "",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_unique_and_the_default_and_whisper_cpp_exist() {
        let mut ids: Vec<_> = catalog().iter().map(|m| m.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), catalog().len());
        assert!(model(DEFAULT_MODEL).is_some());
        assert_eq!(model(WHISPER_CPP).map(|m| m.engine), Some(Engine::WhisperCpp));
    }

    #[test]
    fn ranked_models_come_first_most_accurate_first_down_to_whisper_large_v3() {
        let ranked: Vec<f32> = catalog().iter().map_while(|m| m.leaderboard_wer).collect();
        assert!(ranked.len() >= 13, "the leaderboard models lead the list");
        assert!(ranked.windows(2).all(|w| w[0] <= w[1]), "most accurate first: {ranked:?}");
        let large = catalog().iter().position(|m| m.id == "whisper-large-v3").expect("whisper large-v3");
        assert!(catalog()[..large].iter().all(|m| m.leaderboard_wer.is_some_and(|w| w < 7.44)));
    }

    #[test]
    fn every_transcribe_cpp_model_names_a_gguf_file_on_a_secure_page_and_fits_a_laptop() {
        for m in catalog().iter().filter(|m| m.engine == Engine::TranscribeCpp) {
            assert!(m.page.starts_with("https://huggingface.co/"), "{}", m.id);
            assert!(m.suggested_file.ends_with(".gguf"), "{}", m.id);
            assert!(m.download_mb > 0 && m.download_mb < 4_000, "{} is {} MB", m.id, m.download_mb);
            assert!(!m.languages.is_empty(), "{}", m.id);
        }
    }

    #[test]
    fn language_support_is_read_from_the_model() {
        let english = model("parakeet-tdt-0.6b-v2").expect("parakeet v2");
        assert!(english.speaks("en") && english.speaks("auto") && !english.speaks("es"));
        assert!(model(WHISPER_CPP).expect("whisper.cpp").speaks("es"));
    }
}
