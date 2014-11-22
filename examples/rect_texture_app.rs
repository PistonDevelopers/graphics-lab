
// Allow dead code since this is an example app.
#![allow(dead_code)]
#![feature(globs)]

extern crate shader_version;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{Gl, Texture};
use sdl2_window::Sdl2Window;
use event::{ Events, WindowSettings };

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Rust-Graphics-Lab: Texture App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let assets = Path::new("./examples/assets");
    let image = assets.join(&Path::new("dices.png"));
    let image = Texture::from_path(&image).unwrap();

    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        use graphics::*;
        use event::RenderEvent;
        e.render(|args| {
            gl.viewport(0, 0, args.width as i32, args.height as i32);
            let c = Context::abs(args.width as f64, args.height as f64);
            c.hex("ffaa33").draw(gl);
            c.line(0.0, 100.0, 100.0, 100.0).hex("00ff00").draw(gl);
            c.rect(0.0, 0.0, 50.0, 50.0).rgb(1.0, 0.0, 0.0).draw(gl);
            let offset = 150.0;
            c.trans(0.0, offset).image(&image).draw(gl);
        });
    }
}

