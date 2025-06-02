use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::BufReader;

#[cfg(not(target_arch = "wasm32"))]
use rodio::{Decoder, OutputStream, Sink, Source};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlAudioElement;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

pub struct AudioManager {
    #[cfg(not(target_arch = "wasm32"))]
    _stream: OutputStream,
    #[cfg(not(target_arch = "wasm32"))]
    background_sink: Arc<Mutex<Option<Sink>>>,
    #[cfg(not(target_arch = "wasm32"))]
    effects_sink: Arc<Mutex<Option<Sink>>>,
    
    #[cfg(target_arch = "wasm32")]
    background_audio: RefCell<Option<HtmlAudioElement>>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let background_sink = Sink::try_new(&stream_handle)?;
            let effects_sink = Sink::try_new(&stream_handle)?;
            
            Ok(AudioManager {
                _stream,
                background_sink: Arc::new(Mutex::new(Some(background_sink))),
                effects_sink: Arc::new(Mutex::new(Some(effects_sink))),
            })
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            Ok(AudioManager {
                background_audio: RefCell::new(None),
            })
        }
    }
    
    pub fn start_background_music(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let background_sink = self.background_sink.clone();
            thread::spawn(move || {
                if let Ok(file) = File::open("raw/moments.mp3") {
                    if let Ok(source) = Decoder::new(BufReader::new(file)) {
                        let looped_source = source.repeat_infinite();
                        if let Ok(sink_guard) = background_sink.lock() {
                            if let Some(ref sink) = *sink_guard {
                                sink.append(looped_source);
                                sink.set_volume(0.3); // 设置较低音量作为背景音乐
                                sink.play();
                                println!("Background music started");
                            }
                        }
                    } else {
                        eprintln!("Failed to decode moments.mp3");
                    }
                } else {
                    eprintln!("Failed to open raw/moments.mp3");
                }
            });
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            self.start_web_background_music();
        }
    }
    
    pub fn stop_background_music(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(sink_guard) = self.background_sink.lock() {
                if let Some(ref sink) = *sink_guard {
                    sink.stop();
                }
            }
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            self.stop_web_background_music();
        }
    }
    
    pub fn play_match_sound(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let effects_sink = self.effects_sink.clone();
            thread::spawn(move || {
                // 尝试播放 beep.wav，如果失败则尝试其他格式
                let audio_files = ["raw/beep.wav", "raw/beep.mp3", "raw/beep.ogg"];
                
                for file_path in &audio_files {
                    if let Ok(file) = File::open(file_path) {
                        if let Ok(source) = Decoder::new(BufReader::new(file)) {
                            if let Ok(sink_guard) = effects_sink.lock() {
                                if let Some(ref sink) = *sink_guard {
                                    sink.append(source);
                                    sink.set_volume(0.7); // 音效音量稍高
                                    return; // 成功播放就退出
                                }
                            }
                        }
                    }
                }
                eprintln!("Failed to load any beep sound file");
            });
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            self.play_web_match_sound();
        }
    }
    
    // WASM环境下的音频实现
    #[cfg(target_arch = "wasm32")]
    fn start_web_background_music(&self) {
        use wasm_bindgen::JsCast;
        use web_sys::window;
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Ok(audio) = document.create_element("audio") {
                    let audio = audio.dyn_into::<HtmlAudioElement>().unwrap();
                    audio.set_src("raw/moments.mp3");
                    audio.set_loop(true);
                    audio.set_volume(0.3);
                    
                    // 尝试播放音频
                    let _ = audio.play();
                    
                    // 存储音频引用以便后续控制
                    *self.background_audio.borrow_mut() = Some(audio);
                    
                    web_sys::console::log_1(&"Background music started (WASM)".into());
                }
            }
        }
    }
    
    #[cfg(target_arch = "wasm32")]
    fn stop_web_background_music(&self) {
        if let Some(audio) = self.background_audio.borrow().as_ref() {
            audio.pause().unwrap_or(());
            audio.set_current_time(0.0);
        }
        *self.background_audio.borrow_mut() = None;
        web_sys::console::log_1(&"Background music stopped (WASM)".into());
    }
    
    #[cfg(target_arch = "wasm32")]
    fn play_web_match_sound(&self) {
        use wasm_bindgen::JsCast;
        use web_sys::window;
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Ok(audio) = document.create_element("audio") {
                    let audio = audio.dyn_into::<HtmlAudioElement>().unwrap();
                    audio.set_src("raw/beep.wav");
                    audio.set_volume(0.7);
                    
                    let _ = audio.play();
                    web_sys::console::log_1(&"Match sound played (WASM)".into());
                }
            }
        }
    }
}
