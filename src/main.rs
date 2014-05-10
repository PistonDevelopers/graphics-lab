#![feature(globs)]

extern crate graphics;
extern crate opengles;
extern crate glfw;
extern crate piston;

use piston::*;

mod vector2d;
mod test_polygons;
mod test_colors;
mod snipcode_triangulation;
mod conversion;

mod empty_app;
mod snipcode_triangulation_app;
mod texture_test_app;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    // Create window.
    let window = GameWindow::window("Rust-Graphics-Lab", 300, 300,
        GameWindowSettings {
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0]
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");

    // Create application.
    // let mut app = empty_app::App::new();
    // let mut app = snipcode_triangulation_app::App::new();    
    let mut app = texture_test_app::App::new();

    // Run application.
    app.run(&window, &mut asset_store);
}

