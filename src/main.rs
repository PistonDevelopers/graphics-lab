#![feature(globs)]

extern crate graphics;
extern crate piston;

use piston::*;

mod vector2d;
mod test_polygons;
mod test_colors;
mod triangulation;
mod conversion;

mod empty_app;
mod triangulation_app;
mod texture_test_app;
mod line_app;
mod mouse_app;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    // Create window.
    let mut window: GameWindowGLFW = GameWindow::new(
        GameWindowSettings::new (
            "Rust-Graphics-Lab".to_owned(),
            [300, 300],
            false,
            true,
            [1.0, 1.0, 1.0, 1.0]
        )
    );

    let mut asset_store = AssetStore::from_folder("assets");

    // Create application.
    // let mut app = empty_app::App::new();
    // let mut app = snipcode_triangulation_app::App::new();    
    let mut app = texture_test_app::App::new();
    // let mut app = line_app::App::new();
    // let mut app = mouse_app::App::new();

    // Run application.
    app.run(&mut window, &mut asset_store);
}

