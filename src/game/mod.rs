//! 游戏逻辑模块
//! 
//! 包含游戏的核心逻辑，包括：
//! - 游戏状态管理
//! - 记忆卡片匹配逻辑
//! - 音频回调处理
//! 
//! 此模块负责初始化游戏并处理用户交互。

mod tiles;
use std::{rc::Rc, time::Duration};

use slint::{ComponentHandle, VecModel, Model, Timer};

use crate::{audio::AudioManager, MainWindow};

/// 初始化游戏
/// 
/// 设置游戏状态、绑定回调函数，并准备音频系统。
/// 
/// # 参数
/// - `main_window`: 主窗口引用，用于设置游戏状态和绑定回调
pub fn init(main_window: &MainWindow) {
    // 创建音频管理器
    let audio_manager = Rc::new(AudioManager::new().unwrap_or_else(|_| {
        eprintln!("Failed to initialize audio manager");
        AudioManager::new().unwrap()
    }));

    let tiles_model = std::rc::Rc::new(VecModel::from(tiles::gen()));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();
    let audio_manager_clone = audio_manager.clone();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1 == t2;
            if is_pair_solved {
                t1.solved = true;
                t2.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                tiles_model.set_row_data(t2_idx, t2);
                // 播放匹配成功音效
                audio_manager_clone.play_match_sound();
            } else {
                // Reset the tiles after a short delay
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                Timer::single_shot(Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    t2.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    // 音频回调处理
    let audio_manager_bg = audio_manager.clone();
    main_window.on_start_background_music(move || {
        audio_manager_bg.start_background_music();
        // 在开始游戏时预加载音效，减少播放延迟
        audio_manager_bg.preload_match_sound();
    });

    let audio_manager_stop = audio_manager.clone();
    main_window.on_stop_background_music(move || {
        audio_manager_stop.stop_background_music();
    });

    let audio_manager_match = audio_manager.clone();
    main_window.on_play_match_sound(move || {
        audio_manager_match.play_match_sound();
    });
}
