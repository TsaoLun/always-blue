//! Game logic module
//!
//! Contains core game logic, including:
//! - Game state management
//! - Memory card matching logic
//! - Audio callback handling
//!
//! This module is responsible for initializing the game and handling user interactions.

mod tiles;
use std::{rc::Rc, time::Duration};

use slint::{ComponentHandle, Model, Timer, VecModel};

use crate::{audio::AudioManager, MainWindow, TileData};

/// Initialize game
///
/// Sets up game state, binds callback functions, and prepares audio system.
///
/// # Arguments
/// - `main_window`: Reference to main window, used to set game state and bind callbacks
pub fn init(main_window: &MainWindow) {
    // Create audio manager (automatically degrades to silent mode if initialization fails)
    let audio_manager = Rc::new(AudioManager::new());

    // Lazy load: create empty card model at initialization, load image resources only after game starts
    let tiles_model = Rc::new(VecModel::from(Vec::<TileData>::new()));
    main_window.set_memory_tiles(tiles_model.clone().into());

    // Clone for use in callbacks
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
                // Play match success sound
                audio_manager_clone.play_match_sound();

                // Victory check: check if all cards are matched
                if tiles_model.iter().all(|t| t.solved) {
                    if let Some(w) = main_window_weak_victory.upgrade() {
                        // Delay showing victory screen to let the last pair animation finish
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

    // Game session callback: responsible for initializing/resetting game state and starting audio
    let main_window_weak_start = main_window.as_weak();
    let audio_manager_bg = audio_manager.clone();
    main_window.on_start_game_session(move || {
        // Reset game state
        if let Some(w) = main_window_weak_start.upgrade() {
            w.set_game_completed(false);
            w.set_disable_tiles(false);
        }

        // Lazy load: generate cards and load image resources only when game starts
        let new_tiles = tiles::gen();
        while tiles_model_for_start.row_count() > 0 {
            tiles_model_for_start.remove(tiles_model_for_start.row_count() - 1);
        }
        for tile in new_tiles {
            tiles_model_for_start.push(tile);
        }

        // Audio processing: stop then start to avoid duplicate playback
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
