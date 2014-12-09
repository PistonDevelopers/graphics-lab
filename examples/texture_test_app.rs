
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
use graphics::*;
use event::{
    Events,
    WindowSettings,
    Event,
};

fn main() {
    let opengl = shader_version::opengl::OpenGL::OpenGL_3_2;
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

    let image2 = assets.join(&Path::new("dices.png"));
    let image2 = Texture::from_path(&image2).unwrap();

    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        match e {
            Event::Render(ref args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                let c = Context::abs(args.width as f64, args.height as f64);
                graphics::clear([1.0, ..4], gl);
                let offset = 150.0;
                graphics::image(&image, &c.trans(0.0, offset), gl);
                graphics::image(&image2, &c.trans(offset, 0.0), gl);
                graphics::image(&image2, &c.trans(offset, offset), gl);
            }
            _ => {}
        }
    }
}

