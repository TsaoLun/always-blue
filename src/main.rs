//! 桌面版本入口点
//!
//! 此文件仅用于桌面版本（非WASM）的二进制入口。
//! WASM版本使用lib.rs作为库入口。

slint::include_modules!();

mod audio;
mod game;

/// 桌面版本主函数
///
/// 桌面应用程序的入口点，仅在启用desktop特性时编译。
#[cfg(feature = "desktop")]
fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    // 初始化游戏
    game::init(&main_window);

    // 处理 GitHub 页面打开
    main_window.on_open_github_page(move || {
        if let Err(e) = open::that("https://github.com/tsaolun") {
            eprintln!("Failed to open URL: {}", e);
        }
    });

    main_window.run()
}

/// WASM版本的空主函数
///
/// 在启用wasm特性时，应用程序通过lib.rs中的start_app函数启动。
/// 此函数仅用于满足Rust的二进制入口要求。
#[cfg(feature = "wasm")]
fn main() {
    // WASM版本通过lib.rs中的start_app函数启动
    // 此函数为空，确保编译通过
}
