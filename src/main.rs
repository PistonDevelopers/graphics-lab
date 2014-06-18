#![feature(globs)]

extern crate debug;
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
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Rust-Graphics-Lab".to_owned(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0]
        }
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

