// Allow dead code since this is an example app.
#![allow(dead_code)]
#![feature(globs)]

extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use graphics::*;
use opengl_graphics::{Gl};
use sdl2_game_window::GameWindowSDL2 as Window;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        GameWindowSettings {
            title: "Rust-Graphics-Lab: Empty App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
    let ref mut gl = Gl::new();
    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Render(args) => {
                    gl.viewport(0, 0, args.width as i32, args.height as i32);
                    let c = Context::abs(args.width as f64, args.height as f64);
                    c.rgb(1.0, 1.0, 1.0).draw(gl);
                    // Do rendering here.
                },
                _ => {},       
            },
        }
    }
}

