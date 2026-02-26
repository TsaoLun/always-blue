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
    let memory_tiles = [
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/fish.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/octopus.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/crab.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/jellyfish.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/starfish.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/turtle.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/whale.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
        TileData {
            image: Image::load_from_path(Path::new("ui/icons/seahorse.png")).unwrap_or_default(),
            image_visible: false,
            solved: false,
        },
    ];

    let mut tiles: Vec<TileData> = memory_tiles.to_vec();
    tiles.extend(tiles.clone());

    let mut rng = rand::thread_rng();

    tiles.shuffle(&mut rng);
    tiles
}
