//! WASM库入口点
//!
//! 此文件作为WASM构建的入口点，导出必要的函数供JavaScript调用。
//! 桌面版本使用main.rs作为二进制入口，WASM版本使用此文件作为库入口。

use slint::ComponentHandle;

slint::include_modules!();

mod audio;
mod game;

/// WASM入口函数
///
/// 初始化应用程序并返回主窗口。
/// 此函数由JavaScript调用以启动应用。
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start_app() -> Result<(), wasm_bindgen::JsValue> {
    let main_window = MainWindow::new().map_err(|e| {
        wasm_bindgen::JsValue::from_str(&format!("Failed to create main window: {:?}", e))
    })?;

    // 初始化游戏
    crate::game::init(&main_window);

    // 处理 GitHub 页面打开
    main_window.on_open_github_page(move || {
        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url("https://github.com/tsaolun");
        }
    });

    main_window
        .run()
        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("Application error: {:?}", e)))
}

/// 获取应用版本信息
///
/// 返回当前应用的版本字符串。
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// 测试函数，用于验证WASM模块正常工作
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn test_add(a: i32, b: i32) -> i32 {
    a + b
}
