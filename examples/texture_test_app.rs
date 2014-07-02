
// Allow dead code since this is an example app.
#![allow(dead_code)]
#![feature(globs)]

extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{Gl, Texture};
use Window = sdl2_game_window::GameWindowSDL2;
use graphics::*;
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};

fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Rust-Graphics-Lab: Texture App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let asset_store = AssetStore::from_folder("assets");

    let image = asset_store.path("dices.png").unwrap();
    let image = Texture::from_path(&image).unwrap();

    let image2 = asset_store.path("dices.png").unwrap();
    let image2 = Texture::from_path(&image2).unwrap();

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
    let ref mut gl = Gl::new();
    loop {
        match game_iter.next() {
            None => break,
            Some(mut e) => match e {
                Render(ref mut args) => {
                    gl.viewport(0, 0, args.width as i32, args.height as i32);
                    let c = Context::abs(
                            args.width as f64,
                            args.height as f64
                        );
                    c.rgb(1.0, 1.0, 1.0).draw(gl);
                    let offset = 150.0;
                    c.trans(0.0, offset).image(&image).draw(gl);
                    c.trans(offset, 0.0).image(&image2).draw(gl);
                    c.trans(offset, offset).image(&image2).draw(gl);
                },
                _ => {},
            }
        }
    }
}

