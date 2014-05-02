#![feature(globs)]

extern crate opengles;
extern crate graphics;
extern crate glfw;
extern crate piston;

use GameWindow = piston::game_window::GameWindow;
use Settings = piston::game::Settings;
use Game = piston::game::Game;

mod empty_app;

fn main() {
    let window = GameWindow::window("Rust-Graphics-Lab", 300, 300);
    let mut app = empty_app::EmptyApp::new(
        Settings {
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0]
        }
    );
    app.run(&window);
}
