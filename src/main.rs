slint::include_modules!();

mod audio;
use audio::AudioManager;
use std::rc::Rc;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    use slint::Model;

    // 创建音频管理器
    let audio_manager = Rc::new(AudioManager::new().unwrap_or_else(|_| {
        eprintln!("Failed to initialize audio manager");
        AudioManager::new().unwrap()
    }));

    let main_window = MainWindow::new().unwrap();

    // 准备游戏数据 - 使用 @image-url 宏来加载图片
    let memory_tiles = [
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/fish.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/octopus.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/crab.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/jellyfish.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/starfish.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/turtle.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/whale.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: slint::Image::load_from_path(std::path::Path::new("ui/icons/seahorse.png"))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
    ];

    let mut tiles: Vec<TileData> = memory_tiles.to_vec();
    tiles.extend(tiles.clone());

    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
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
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
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

    // 处理 GitHub 页面打开
    main_window.on_open_github_page(move || {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .unwrap()
                .open_with_url("https://github.com/tsaolun")
                .unwrap();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Err(e) = open::that("https://github.com/tsaolun") {
                eprintln!("Failed to open URL: {}", e);
            }
        }
    });

    main_window.run().unwrap();
}
