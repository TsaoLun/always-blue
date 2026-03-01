//! Audio management module
//!
//! Provides cross-platform audio playback capabilities, supporting:
//! - Desktop platform: uses rodio library
//! - WASM platform: uses Web Audio API
//!
//! The module automatically selects the appropriate implementation based on the target platform.

// Standard library imports
use std::sync::{Arc, Mutex};

// Platform specific imports
#[cfg(feature = "desktop")]
use {
    rodio::{Decoder, OutputStream, Sink, Source},
    std::fs::File,
    std::io::BufReader,
    std::thread,
};

#[cfg(feature = "wasm")]
use {std::cell::RefCell, web_sys::HtmlAudioElement};

/// Audio Manager
///
/// Responsible for managing all audio playback in the game, including:
/// - Background music
/// - Sound effects (match success sound, etc.)
///
/// Automatically selects the appropriate backend implementation based on compilation target.
pub struct AudioManager {
    #[cfg(feature = "desktop")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "desktop")]
    background_sink: Arc<Mutex<Option<Sink>>>,
    #[cfg(feature = "desktop")]
    effects_sink: Arc<Mutex<Option<Sink>>>,

    #[cfg(feature = "wasm")]
    background_audio: RefCell<Option<HtmlAudioElement>>,
    #[cfg(feature = "wasm")]
    match_sound_audio: RefCell<Option<HtmlAudioElement>>,
}

impl AudioManager {
    /// Create a new audio manager
    ///
    /// # Returns
    /// - `AudioManager`: Audio manager instance. Returns a silent mode manager if audio system is unavailable.
    pub fn new() -> Self {
        #[cfg(feature = "desktop")]
        {
            match OutputStream::try_default() {
                Ok((_stream, stream_handle)) => {
                    let background_sink = Sink::try_new(&stream_handle).ok();
                    let effects_sink = Sink::try_new(&stream_handle).ok();
                    AudioManager {
                        _stream: Some(_stream),
                        background_sink: Arc::new(Mutex::new(background_sink)),
                        effects_sink: Arc::new(Mutex::new(effects_sink)),
                    }
                }
                Err(e) => {
                    eprintln!("Audio system unavailable: {}", e);
                    AudioManager {
                        _stream: None,
                        background_sink: Arc::new(Mutex::new(None)),
                        effects_sink: Arc::new(Mutex::new(None)),
                    }
                }
            }
        }

        #[cfg(feature = "wasm")]
        {
            AudioManager {
                background_audio: RefCell::new(None),
                match_sound_audio: RefCell::new(None),
            }
        }

        #[cfg(not(any(feature = "desktop", feature = "wasm")))]
        {
            compile_error!("Must enable either 'desktop' or 'wasm' feature");
        }
    }

    /// Start playing background music
    ///
    /// Background music loops in the background with volume set to 30%.
    /// In WASM environment, it automatically creates HTML5 audio elements.
    pub fn start_background_music(&self) {
        #[cfg(feature = "desktop")]
        {
            let background_sink = self.background_sink.clone();
            thread::spawn(move || {
                let Ok(file) = File::open("raw/moments.mp3") else {
                    eprintln!("Failed to open raw/moments.mp3");
                    return;
                };

                let Ok(source) = Decoder::new(BufReader::new(file)) else {
                    eprintln!("Failed to decode moments.mp3");
                    return;
                };

                let looped_source = source.repeat_infinite();

                let Ok(sink_guard) = background_sink.lock() else {
                    return;
                };

                let Some(ref sink) = *sink_guard else {
                    return;
                };

                sink.append(looped_source);
                sink.set_volume(0.3); // Set lower volume for background music
                sink.play();
                println!("Background music started");
            });
        }

        #[cfg(feature = "wasm")]
        {
            self.start_web_background_music();
        }
    }

    /// Stop playing background music
    ///
    /// Stops the currently playing background music.
    /// In WASM environment, it pauses and resets the audio element.
    pub fn stop_background_music(&self) {
        #[cfg(feature = "desktop")]
        {
            let Ok(sink_guard) = self.background_sink.lock() else {
                return;
            };

            let Some(ref sink) = *sink_guard else {
                return;
            };

            sink.stop();
        }

        #[cfg(feature = "wasm")]
        {
            self.stop_web_background_music();
        }
    }

    /// Play match success sound effect
    ///
    /// Plays a prompt sound effect when a match is successful, volume set to 70%.
    /// Supports multiple audio formats (wav, mp3, ogg), trying to load in order.
    pub fn play_match_sound(&self) {
        #[cfg(feature = "desktop")]
        {
            let effects_sink = self.effects_sink.clone();
            thread::spawn(move || {
                // Try playing beep.wav, if failed try other formats
                let audio_files = ["raw/beep.wav", "raw/beep.mp3", "raw/beep.ogg"];

                for file_path in &audio_files {
                    let Ok(file) = File::open(file_path) else {
                        continue;
                    };

                    let Ok(source) = Decoder::new(BufReader::new(file)) else {
                        continue;
                    };

                    let Ok(sink_guard) = effects_sink.lock() else {
                        continue;
                    };

                    let Some(ref sink) = *sink_guard else {
                        continue;
                    };

                    sink.append(source);
                    sink.set_volume(0.7); // Sound effect volume slightly higher
                    return; // Exit on success
                }
                eprintln!("Failed to load any beep sound file");
            });
        }

        #[cfg(feature = "wasm")]
        {
            self.play_web_match_sound();
        }
    }

    // Preload sound effects (WASM environment)
    #[cfg(feature = "wasm")]
    /// Preload match sound effect (WASM environment only)
    ///
    /// Preloads sound effect files in WASM environment to reduce latency on first play.
    /// This method is a no-op in desktop environment.
    pub fn preload_match_sound(&self) {
        use wasm_bindgen::JsCast;
        use web_sys::window;

        let Some(window) = window() else {
            return;
        };

        let Some(document) = window.document() else {
            return;
        };

        let Ok(audio) = document.create_element("audio") else {
            return;
        };

        let audio = audio.dyn_into::<HtmlAudioElement>().unwrap();
        audio.set_src("raw/beep.wav");
        audio.set_volume(0.7);
        audio.set_preload("auto"); // Preload audio

        // Store preloaded audio reference
        *self.match_sound_audio.borrow_mut() = Some(audio);

        web_sys::console::log_1(&"Match sound preloaded (WASM)".into());
    }

    // Empty implementation for non-WASM environment to maintain API consistency
    #[cfg(feature = "desktop")]
    pub fn preload_match_sound(&self) {
        // Non-WASM environments don't need preloading
    }

    // Audio implementation for WASM environment
    #[cfg(feature = "wasm")]
    fn start_web_background_music(&self) {
        use wasm_bindgen::JsCast;
        use web_sys::window;

        let Some(window) = window() else {
            return;
        };

        let Some(document) = window.document() else {
            return;
        };

        let Ok(audio) = document.create_element("audio") else {
            return;
        };

        let audio = audio.dyn_into::<HtmlAudioElement>().unwrap();
        audio.set_src("raw/moments.mp3");
        audio.set_loop(true);
        audio.set_volume(0.3);

        // Try playing audio
        let _ = audio.play();

        // Store audio reference for later control
        *self.background_audio.borrow_mut() = Some(audio);

        web_sys::console::log_1(&"Background music started (WASM)".into());
    }

    #[cfg(feature = "wasm")]
    fn stop_web_background_music(&self) {
        if let Some(audio) = self.background_audio.borrow().as_ref() {
            audio.pause().unwrap_or(());
            audio.set_current_time(0.0);
        }
        *self.background_audio.borrow_mut() = None;
        web_sys::console::log_1(&"Background music stopped (WASM)".into());
    }

    #[cfg(feature = "wasm")]
    fn play_web_match_sound(&self) {
        // First try using preloaded audio
        if let Some(audio) = self.match_sound_audio.borrow().as_ref() {
            // Reset audio position and play
            audio.set_current_time(0.0);
            let _ = audio.play();
            web_sys::console::log_1(&"Match sound played from preloaded audio (WASM)".into());
            return;
        }

        // If no preloaded audio, create new audio element (fallback)
        use wasm_bindgen::JsCast;
        use web_sys::window;

        let Some(window) = window() else {
            return;
        };

        let Some(document) = window.document() else {
            return;
        };

        let Ok(audio) = document.create_element("audio") else {
            return;
        };

        let audio = audio.dyn_into::<HtmlAudioElement>().unwrap();
        audio.set_src("raw/beep.wav");
        audio.set_volume(0.7);

        let _ = audio.play();
        web_sys::console::log_1(&"Match sound played with new audio element (WASM)".into());
    }
}
