#![feature(globs)]

extern crate opengles;
extern crate graphics;
extern crate glfw;
extern crate piston;

use piston::*;

mod vector2d;
mod empty_app;
mod test_polygons;
mod snipcode_triangulation;
mod conversion;

fn main() {
    // Create window.
    let window = GameWindow::window("Rust-Graphics-Lab", 300, 300,
        GameWindowSettings {
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0]
        }
    );

    // Create application.
    let mut app = empty_app::EmptyApp::new();
    
    // Run application.
    app.run(&window);
}

