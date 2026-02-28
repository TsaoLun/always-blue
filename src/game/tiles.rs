//! 记忆卡片生成模块
//!
//! 负责生成游戏中的记忆卡片，包括：
//! - 加载卡片图片
//! - 创建卡片对
//! - 随机打乱卡片顺序

use std::path::Path;

use crate::TileData;
use rand::seq::SliceRandom;
use slint::Image;

/// 生成记忆卡片
///
/// 创建8对不同的海洋生物卡片，每对包含相同的图片。
/// 卡片会被随机打乱顺序，确保每次游戏都有不同的布局。
///
/// # 返回
/// - `Vec<TileData>`: 包含16张卡片的向量，每对卡片有相同的图片
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
