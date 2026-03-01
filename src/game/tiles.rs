//! Memory card generation module
//!
//! Responsible for generating memory cards in the game, including:
//! - Loading card images
//! - Creating card pairs
//! - Randomly shuffling card order

use std::path::Path;

use crate::TileData;
use rand::seq::SliceRandom;
use slint::Image;

/// Generate memory cards
///
/// Creates 8 pairs of different ocean creature cards, each pair containing the same image.
/// Cards are randomly shuffled to ensure a different layout for each game.
///
/// # Returns
/// - `Vec<TileData>`: Vector containing 16 cards, each pair has the same image
pub fn gen() -> Vec<TileData> {
    let icon_names = [
        "fish", "octopus", "crab", "jellyfish",
        "starfish", "turtle", "whale", "seahorse",
    ];

    let memory_tiles: Vec<TileData> = icon_names
        .iter()
        .map(|name| TileData {
            image: Image::load_from_path(Path::new(&format!("ui/icons/{name}.png")))
                .unwrap_or_default(),
            image_visible: false,
            solved: false,
        })
        .collect();

    let mut tiles = memory_tiles.clone();
    tiles.extend(memory_tiles);

    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);
    tiles
}
