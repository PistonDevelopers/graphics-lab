#![feature(globs)]

extern crate opengles;
extern crate graphics;
extern crate glfw;
extern crate piston;

use piston::*;

mod vector2d;
mod test_polygons;
mod snipcode_triangulation;
mod conversion;

mod empty_app;
mod snipcode_triangulation_app;

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

    // Create application.
    // let mut app = empty_app::App::new();
    let mut app = snipcode_triangulation_app::App::new();    

    // Run application.
    app.run(&window);
}

