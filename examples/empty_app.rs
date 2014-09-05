// Allow dead code since this is an example app.
#![allow(dead_code)]
#![feature(globs)]

extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use graphics::*;
use opengl_graphics::{Gl};
use sdl2_game_window::WindowSDL2;
use piston::{
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
};

fn main() {
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "Rust-Graphics-Lab: Empty App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut event_iter = EventIterator::new(&mut window, &event_settings);
    let ref mut gl = Gl::new();
    for e in event_iter {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                // Do rendering here.
            }
            _ => {}
        }
    }
}

