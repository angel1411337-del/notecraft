# Dictate into a page

Dictation is an optional native feature. Build with:

```sh
cargo build -p notecraft --features voice
```

Choose **Dictate** (the microphone beside the ribbon tabs) or press **F8**. The first
time, **Dictation setup** opens: choose a **Speech model** from the list, the program
that runs it (`transcribe-cli` from transcribe.cpp, or `whisper-cli` from whisper.cpp)
and the model file, then use **Test microphone** and **Test recognizer** to check both
halves. The program and model file must already exist on your computer. NoteCraft does
not download them, contact a hosted provider, or require an account. Setup stays
available from the `...` button beside **Dictate** and from **File > Options**.
Linux builds need the CPAL/ALSA development dependencies. On macOS,
`bash packaging/OSX/package.sh --voice` builds an optional voice-enabled app bundle
with the microphone usage description and hardened runtime audio-input entitlement.

Choose the language in setup: **Detect automatically**, English, Spanish, and others.
Setup shows how many languages the chosen model transcribes, and warns (and does not
start) when it does not list the chosen language. For **Your own whisper.cpp model**,
NoteCraft cannot tell: models whose name ends in `.en` understand English only.

## Choose a speech model

The list is ranked by the average word error rate (WER, lower is better) on the Hugging
Face [Open ASR Leaderboard](https://huggingface.co/spaces/hf-audio/open_asr_leaderboard),
down to Whisper large-v3, keeping only models a local program can run on a laptop. All of
them run in [transcribe.cpp](https://github.com/handy-computer/transcribe.cpp)'s
`transcribe-cli` from a GGUF file. Setup links each model's page and names the file to
download (the Q4_K_M build). Any GGUF file you choose will run; the list only guides.

| Model | WER | Download | Languages | License |
| --- | --- | --- | --- | --- |
| Granite Speech 4.1 2B (IBM) | 5.33% | 1.6 GB | 6 | Apache-2.0 |
| Cohere Transcribe (Cohere) | 5.42% | 1.6 GB | 14 | Apache-2.0 |
| Granite 4.0 1B Speech (IBM) | 5.52% | 1.6 GB | 6 | Apache-2.0 |
| Canary-Qwen 2.5B (NVIDIA) | 5.63% | 1.7 GB | 1 | CC-BY-4.0 |
| Qwen3-ASR 1.7B (Alibaba) | 5.76% | 1.3 GB | 30 | Apache-2.0 |
| Parakeet Unified 0.6B (NVIDIA) | 5.91% | 480 MB | 1 | NVIDIA Open Model License |
| Parakeet TDT 0.6B v2 (NVIDIA) | 6.05% | 480 MB | 1 | CC-BY-4.0 |
| Parakeet TDT 0.6B v3 (NVIDIA), the default | 6.34% | 490 MB | 25 | CC-BY-4.0 |
| Canary 1B (NVIDIA) | 6.50% | 730 MB | 4 | CC-BY-NC-4.0 (non-commercial) |
| Nemotron Speech Streaming 0.6B (NVIDIA) | 6.93% | 480 MB | 1 | NVIDIA Open Model License |
| Parakeet TDT 1.1B (NVIDIA) | 7.02% | 830 MB | 1 | CC-BY-4.0 |
| Voxtral Mini 3B (Mistral AI) | 7.05% | 3.0 GB | 8 | Apache-2.0 |
| Whisper large-v3 (OpenAI) | 7.44% | 1.0 GB | 100 | Apache-2.0 |
| Whisper large-v3 turbo (OpenAI) | 7.83% | 540 MB | 100 | MIT |
| Whisper small, Whisper base (OpenAI) | not ranked | 170 MB, 60 MB | 99 | Apache-2.0 |
| Your own whisper.cpp model | - | - | - | - |

Left out: Granite Speech 3.3 8B and Phi-4 Multimodal (no local program runs them) and
Voxtral Small 24B (too large for a laptop). The default, Parakeet TDT 0.6B v3, is small and fast on a laptop processor and is the most
accurate of those that speak more than English (25 languages).
**Your own whisper.cpp model** keeps the first setup: a `whisper-cli` and a ggml model.

Sources, as of November 2026: WER from the leaderboard as reported by
[Codesota](https://codesota.com/speech/stt-leaderboard) (May 2026 snapshot) and, for
Parakeet Unified, Parakeet TDT v3 and Nemotron, by
[OpenWhispr](https://openwhispr.com/blog/parakeet-vs-whisper-vs-nemotron) (July 2026).
Sizes, languages and licenses from the transcribe.cpp model catalog. Leaderboard audio is
fairly clean; results on your voice and microphone differ. Rankings change; the list is
data in `crates/voice-core/src/models.rs`.

1. Click where the words should go. With no cursor (on a new page, say), or with the
   cursor in a table, code or an equation, the words become a new paragraph at the end
   of the page.
2. Choose **Dictate** or press **F8**, and speak. A bar at the bottom shows **Listening**,
   a level meter and the time. Tap F8 to start and again to finish, or hold it while you
   talk. **Enter** or **Done** finishes; **Esc** or **Cancel** stops and discards.
3. A preview of the words appears under the line they will go into, revised every few
   seconds, then replaced by the final pass when you finish.
4. Review the words, choose **Edit** to correct them, then **Insert here**, or
   **Discard**. Lines become separate paragraphs. Undo takes the insertion back.
   Turn off **Show the words for review before inserting them** in setup to insert as
   soon as the final pass is done.

One recording lasts at most 60 seconds. The transcript is limited to 64 KiB.
Source view and read-only pages cannot take dictation. Dictation is literal text; it
does not interpret spoken commands or reformat notes.

If you edit the page while the words are being spoken or reviewed, **Insert here** puts
them where your cursor is at that moment and says so. If you open a different page
before inserting, the words are never moved there on their own, not even with review
turned off: the preview names the page they were for and offers **Insert on this page**.
If another program changes the page's file, reload the page and dictate again.

With review turned off, only a finished final pass for an unchanged page and selection
goes in by itself. Moving the cursor or editing the page keeps the words for explicit
review; Insert can then choose the current cursor position. Words left from a
recording that failed (a disconnected microphone, say) wait for review. A held shortcut
ends the recording when the key comes up or when NoteCraft loses focus.

Dictation inserts literal Span text and checks that the page format can save and reload
it unchanged before applying an edit. Existing readers and writers do not preserve
every punctuation combination. Backslashes, paired emphasis markers and tag-like
paragraph starts may be rejected. The draft stays visible for review and copying.
Edit the draft to remove unsupported punctuation before inserting. Dictation never
adds hidden characters or silently interprets the draft as formatting.

Problems are explained in the bar with something to do about them: **Microphone
settings** opens the system's microphone privacy page (Windows and macOS), and
**Dictation setup** opens setup.

Audio and transcripts are session-only until you choose Insert. The process
adapters temporarily write WAV and transcript files in a private `voice-cache` folder
beside local settings, outside notebooks. It deletes job files on completion,
cancellation and failure. A later job cleans abandoned inactive files after an
interrupted process. Cleanup is best effort and is not secure erasure. Do not
put the settings/cache folder in a synced location. Raw speech and note contents
are not written to diagnostics by this feature.

Recognition runs in background workers, one process at a time, with a default
120-second timeout per inference. Preview recognition starts after five seconds
and reloads the model for each pass. This is chunked preview, not token streaming.
A preview that fails is skipped; only the final pass can end a recording with an
error.

The microphone opens as soon as you press the talk key. The recognizer's `--help`
check runs alongside it, once per executable while the app is open, so the first
words are not lost. On Windows the recognizer runs without a console window.
When whisper-cli advertises `-sns`, non-speech tokens are suppressed, and
known silence labels such as `[BLANK_AUDIO]` or `(music)` are dropped; other words
in brackets are kept for review. transcribe-cli runs with `-q -m MODEL -o FILE` and
`-l LANGUAGE` unless the language is automatic; NoteCraft reads only the transcript
file, drops lines that are only a silence label, and checks that the model file is GGUF
before recording.

The panel shows RECORDING only once the microphone delivers audio, with an input
level meter. If the device sends nothing for five seconds, including a stream that stalls after
recording started, recording stops with a
message. If audio arrives but stays silent for two seconds, the panel says so: check
the selected microphone, mute switches, and the system's microphone permission. Clips shorter
than a second are padded with silence; if nothing is recognized, the panel says so.
Inserted words get a space where they meet a word already on the line.
Multiple lines cannot be inserted before indexed table widths or paragraph justification
would move: use one line there, or click below the page to append all the words.
A trusted but faulty recognizer can grow its output file until timeout; reads into
memory are bounded. Only select an executable you trust.

The engine registers `voice.start`, `voice.stop`, `voice.cancel`,
`voice.draft.set`, `voice.retarget`, `voice.commit`, and `voice.status`.
Microphone and edit commands require the local UI invocation context. CLI and MCP
cannot grant themselves that context using parameters. Status exposes lifecycle
metadata only. Existing trusted UI automation can still click native controls;
this does not add a separate security boundary around an already trusted operator.

Conversation, notebook retrieval through speech, live hosted voice adapters,
speech synthesis, and browser capture are later milestones. An MCP server alone
does not establish compatibility with a proprietary application's voice mode.

## Verification limits

Automated tests cover session revisions, literal insertion, both page formats,
selection, normal history/save integration, stale targets, canonical cache paths,
audio conversion and real process cancellation. Those tests do not establish
microphone permission behavior, recognition accuracy, or language support.
Real microphone/model acceptance and Windows/macOS device testing remain required.

## Manual native acceptance

On Linux, Windows and macOS, test a real microphone with a real local transcribe-cli and
GGUF model, and with a whisper-cli and ggml model. Record the OS, device, recognizer
version, model and language with the result.

0. Before the app, run a model on a recording with the runtime example. The program's
   file name picks the adapter (`transcribe-cli` or `whisper-cli`):

   ```sh
   cargo run -p craft-voice-runtime --example transcribe -- /path/to/transcribe-cli /path/to/parakeet-tdt-0.6b-v3-Q4_K_M.gguf /path/to/recording.wav /path/to/private-voice-jobs
   ```


1. Test microphone in setup. Speak, mute it and select another device. Check the live
   meter, silence warning, disconnect error and retry after reconnecting.
2. Deny system microphone access. Start dictation and check a permission or no-audio
   error within five seconds after stream opening. Re-enable access and retry. On
   macOS, test the packaged app; terminal consent is a separate check.
3. Dictate short sentences in two model-supported languages. Check previews, final
   review, edits, Undo/Redo, save and reload in HTML and CMGFM. Try a very short press
   and the 60-second limit. Try punctuation that the format cannot preserve.
4. Cancel during startup, listening, preview inference and final inference. Check the
   microphone indicator turns off, recognizer exits and job files disappear. Retry
   after each cancellation and after an invalid-model error.
5. Move the caret, select other text, edit the page, switch pages and change the file
   externally while decoding. Check automatic insertion never retargets silently,
   rejected edits retain review and explicit insertion requires a valid target.
6. Tap and hold the shortcut; release it while another window has focus. Check repeat
   keys and modifiers. Setup and dialogs must not start a microphone behind them.

An opt-in test checks actual CPAL callbacks, two seconds of non-silent sample data,
and reopening after cleanup. Speak during the test:

```sh
cargo test -p craft-voice-runtime --features microphone actual_microphone_delivers_samples_and_releases_the_device -- --ignored --nocapture --test-threads=1
```

`VOICE_CAPTURE_DEVICE` can select an exact CPAL device name. Native virtual devices
may serve recorded or synthetic audio; label that evidence as virtual input. It does
not verify a physical microphone, acoustic fidelity or OS consent prompts.

New interface messages use English and pseudo-language catalogs. Real locales fall
back to English until reviewed translations are supplied.

Multiple paragraphs are inserted only at a paragraph end or below the page. Inserting
several lines before existing words is refused so those words cannot move ahead of the
new transcript. The words stay in review.
