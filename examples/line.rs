
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
use event::{ Events, WindowSettings };

fn render(c: &graphics::Context, gl: &mut Gl) {
    use graphics::*;

    // Create a line.
    let line = graphics::Line::round([1.0, 1.0, 0.0, 1.0], 3.0);

    // Draw ten lines beside each other with hue transformed color.
    let n = 10;
    let (start, end) = (0.0, 400.0);
    for i in range(0u, n + 1) {
        let f = i as f64 / n as f64;
        line
            .hue_deg(f as f32 * 360.0)
            .draw([0.0, 0.0, 0.0, 100.0], 
                &c.trans(f * (end - start) + start, 0.0), gl);
    }
}

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
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
            use graphics::*;

            gl.viewport(0, 0, args.width as i32, args.height as i32);
            let c = Context::abs(args.width as f64, args.height as f64);
            graphics::clear([1.0, ..4], gl);
            render(&c, gl);
        });
    }
}
