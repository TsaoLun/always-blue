//! 音频管理模块
//!
//! 提供跨平台的音频播放功能，支持：
//! - 桌面平台：使用rodio库
//! - WASM平台：使用Web Audio API
//!
//! 模块自动根据目标平台选择适当的实现。

// 标准库导入
use std::sync::{Arc, Mutex};

// 平台特定导入
#[cfg(feature = "desktop")]
use {
    rodio::{Decoder, OutputStream, Sink, Source},
    std::fs::File,
    std::io::BufReader,
    std::thread,
};

#[cfg(feature = "wasm")]
use {std::cell::RefCell, web_sys::HtmlAudioElement};

/// 音频管理器
///
/// 负责管理游戏中的所有音频播放，包括：
/// - 背景音乐
/// - 音效（匹配成功音效等）
///
/// 根据编译目标自动选择适当的后端实现。
pub struct AudioManager {
    #[cfg(feature = "desktop")]
    _stream: OutputStream,
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
    /// 创建新的音频管理器
    ///
    /// # 返回
    /// - `Ok(AudioManager)`: 成功创建的音频管理器
    /// - `Err(Box<dyn std::error::Error>)`: 初始化失败的错误
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "desktop")]
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

        #[cfg(feature = "wasm")]
        {
            Ok(AudioManager {
                background_audio: RefCell::new(None),
                match_sound_audio: RefCell::new(None),
            })
        }

        #[cfg(not(any(feature = "desktop", feature = "wasm")))]
        {
            compile_error!("必须启用 'desktop' 或 'wasm' 特性之一");
        }
    }

    /// 开始播放背景音乐
    ///
    /// 背景音乐会在后台循环播放，音量设置为30%。
    /// 在WASM环境中，会自动创建HTML5音频元素。
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
                sink.set_volume(0.3); // 设置较低音量作为背景音乐
                sink.play();
                println!("Background music started");
            });
        }

        #[cfg(feature = "wasm")]
        {
            self.start_web_background_music();
        }
    }

    /// 停止播放背景音乐
    ///
    /// 停止当前正在播放的背景音乐。
    /// 在WASM环境中，会暂停并重置音频元素。
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

    /// 播放匹配成功音效
    ///
    /// 播放匹配成功时的提示音效，音量设置为70%。
    /// 支持多种音频格式（wav, mp3, ogg），按顺序尝试加载。
    pub fn play_match_sound(&self) {
        #[cfg(feature = "desktop")]
        {
            let effects_sink = self.effects_sink.clone();
            thread::spawn(move || {
                // 尝试播放 beep.wav，如果失败则尝试其他格式
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
                    sink.set_volume(0.7); // 音效音量稍高
                    return; // 成功播放就退出
                }
                eprintln!("Failed to load any beep sound file");
            });
        }

        #[cfg(feature = "wasm")]
        {
            self.play_web_match_sound();
        }
    }

    // 预加载音效（WASM环境）
    #[cfg(feature = "wasm")]
    /// 预加载匹配音效（仅WASM环境）
    ///
    /// 在WASM环境中预加载音效文件，减少首次播放时的延迟。
    /// 桌面环境此方法为空操作。
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
        audio.set_preload("auto"); // 预加载音频

        // 存储预加载的音频引用
        *self.match_sound_audio.borrow_mut() = Some(audio);

        web_sys::console::log_1(&"Match sound preloaded (WASM)".into());
    }

    // 非WASM环境的空实现，保持API一致性
    #[cfg(feature = "desktop")]
    pub fn preload_match_sound(&self) {
        // 非WASM环境不需要预加载
    }

    // WASM环境下的音频实现
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

        // 尝试播放音频
        let _ = audio.play();

        // 存储音频引用以便后续控制
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
        // 首先尝试使用预加载的音频
        if let Some(audio) = self.match_sound_audio.borrow().as_ref() {
            // 重置音频位置并播放
            audio.set_current_time(0.0);
            let _ = audio.play();
            web_sys::console::log_1(&"Match sound played from preloaded audio (WASM)".into());
            return;
        }

        // 如果没有预加载音频，则创建新的音频元素（兜底方案）
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
