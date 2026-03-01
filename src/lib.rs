//! WASM Library Entry Point
//!
//! This file is the entry point for WASM builds, exporting necessary functions for JavaScript calls.
//! The desktop version uses main.rs as the binary entry point, while the WASM version uses this file as the library entry point.

use slint::ComponentHandle;

slint::include_modules!();

mod audio;
mod game;

/// WASM entry function
///
/// Initializes the application and returns the main window.
/// This function is called by JavaScript to start the app.
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start_app() -> Result<(), wasm_bindgen::JsValue> {
    let main_window = MainWindow::new().map_err(|e| {
        wasm_bindgen::JsValue::from_str(&format!("Failed to create main window: {:?}", e))
    })?;

    // Initialize game
    crate::game::init(&main_window);

    // Handle GitHub page open
    main_window.on_open_github_page(move || {
        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url("https://github.com/tsaolun");
        }
    });

    main_window
        .run()
        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("Application error: {:?}", e)))
}

/// Get application version information
///
/// Returns the current application version string.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Test function to verify WASM module works correctly
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn test_add(a: i32, b: i32) -> i32 {
    a + b
}
