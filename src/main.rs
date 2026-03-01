//! Desktop entry point
//!
//! This file is only used for the desktop version (non-WASM) binary entry point.
//! The WASM version uses lib.rs as the library entry point.

slint::include_modules!();

mod audio;
mod game;

/// Desktop version main function
///
/// The entry point for desktop applications, compiled only when the desktop feature is enabled.
#[cfg(feature = "desktop")]
fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    // Initialize game
    game::init(&main_window);

    // Handle GitHub page opening (main page -> profile)
    main_window.on_open_github_page(move || {
        if let Err(e) = open::that("https://github.com/tsaolun") {
            eprintln!("Failed to open URL: {}", e);
        }
    });

    // Handle GitHub repo opening (victory screen -> repo)
    main_window.on_open_github_repo(move || {
        if let Err(e) = open::that("https://github.com/tsaolun/always-blue") {
            eprintln!("Failed to open URL: {}", e);
        }
    });

    main_window.run()
}

/// Empty main function for WASM version
///
/// When the wasm feature is enabled, the application starts via the start_app function in lib.rs.
/// This function is only used to satisfy Rust's binary entry requirement.
#[cfg(feature = "wasm")]
fn main() {
    // The WASM version starts via the start_app function in lib.rs
    // This function is empty ensuring compilation passes
}
