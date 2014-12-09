
// Allow dead code since this is an example app.
#![allow(dead_code)]
#![feature(globs)]

extern crate shader_version;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{Gl};
use sdl2_window::Sdl2Window;
use graphics::*;
use event::{ Events, WindowSettings };

fn render(c: &Context, gl: &mut Gl) { 
    // c.ellipse(0.0, 0.0, 100.0, 200.0).rgb(1.0, 0.0, 0.0).border_width(3.0).draw(gl);

    // c.rect(0.0, 0.0, 100.0, 100.0).round(20.0).rgb(1.0, 0.0, 0.0).border_width(3.0).draw(gl);

    // c.rect(0.0, 0.0, 100.0, 100.0).bevel(20.0).rgb(1.0, 0.0, 0.0).border_width(3.0).draw(gl);

    graphics::Rectangle::border([1.0, 0.0, 0.0, 1.0], 5.0)
        .draw([20.0, 20.0, 220.0, 200.0], c, gl);
}

fn main() {
    let opengl = shader_version::opengl::OpenGL::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Rust-Graphics-Lab: Line App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        use event::RenderEvent;
        e.render(|args| {
            gl.viewport(0, 0, args.width as i32, args.height as i32);
            let c = Context::abs(args.width as f64, args.height as f64);
            graphics::clear([1.0, ..4], gl);
            render(&c, gl);
        });
    }
}
