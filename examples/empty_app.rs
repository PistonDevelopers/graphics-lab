// Allow dead code since this is an example app.
#![allow(dead_code)]

extern crate piston;
extern crate graphics;

use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSDL2,
    GameWindowSettings,
    Render,
};

fn start(argc: int, argv: **u8) -> int {
    // Run on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Rust-Graphics-Lab: Empty App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0]
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Render(_args) => {
                    // Do rendering here.
                },
                _ => {},       
            },
        }
    }
}

