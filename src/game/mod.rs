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

use slint::{ComponentHandle, Model, Timer, VecModel};

use crate::{audio::AudioManager, MainWindow, TileData};

/// 初始化游戏
///
/// 设置游戏状态、绑定回调函数，并准备音频系统。
///
/// # 参数
/// - `main_window`: 主窗口引用，用于设置游戏状态和绑定回调
pub fn init(main_window: &MainWindow) {
    // 创建音频管理器（初始化失败时自动降级为静默模式）
    let audio_manager = Rc::new(AudioManager::new());

    // 懒加载：初始化时创建空的卡片模型，开始游戏后才加载图片资源
    let tiles_model = Rc::new(VecModel::from(Vec::<TileData>::new()));
    main_window.set_memory_tiles(tiles_model.clone().into());

    // 克隆供各回调使用
    let tiles_model_for_start = tiles_model.clone();

    let main_window_weak = main_window.as_weak();
    let main_window_weak_victory = main_window.as_weak();
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

                // 胜利检测：检查是否所有卡片都已匹配
                if tiles_model.iter().all(|t| t.solved) {
                    if let Some(w) = main_window_weak_victory.upgrade() {
                        // 延迟显示胜利界面，让最后一对匹配动画完成
                        Timer::single_shot(Duration::from_millis(500), move || {
                            w.set_game_completed(true);
                        });
                    }
                }
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

    // 游戏会话回调：负责初始化/重置游戏状态并启动音频
    let main_window_weak_start = main_window.as_weak();
    let audio_manager_bg = audio_manager.clone();
    main_window.on_start_game_session(move || {
        // 重置游戏状态
        if let Some(w) = main_window_weak_start.upgrade() {
            w.set_game_completed(false);
            w.set_disable_tiles(false);
        }

        // 懒加载：开始游戏时才生成卡片并加载图片资源
        let new_tiles = tiles::gen();
        while tiles_model_for_start.row_count() > 0 {
            tiles_model_for_start.remove(tiles_model_for_start.row_count() - 1);
        }
        for tile in new_tiles {
            tiles_model_for_start.push(tile);
        }

        // 音频处理：先停止再启动，避免重复播放
        audio_manager_bg.stop_background_music();
        audio_manager_bg.start_background_music();
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
