#![allow(dead_code)]
#![feature(globs)]

extern crate input;
extern crate shader_version;
extern crate event;
extern crate graphics;
extern crate "rust-graphics-lab" as lab;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use sdl2_window::Sdl2Window;
use opengl_graphics::{Gl};
use graphics::*;
use event::{
    Events,
    RenderArgs,
    WindowSettings,
};
use graphics::modular_index::{offset};
use input::keyboard;

// Local crate.
use lab::test_colors;
use lab::test_polygons;
use lab::conversion;
use lab::triangulation as triangulation_lib;

pub struct App {
    test_polygon_index: uint,
    gl: Gl,
}

impl App {
    pub fn new(opengl: shader_version::opengl::OpenGL) -> App {
        App {
            test_polygon_index: 0,
            gl: Gl::new(opengl),
        }
    }

    fn draw(&mut self, args: &RenderArgs) {
        let ref mut gl = self.gl;
        gl.viewport(0, 0, args.width as i32, args.height as i32);
        let c = Context::abs(args.width as f64, args.height as f64);
        c.rgb(1.0, 1.0, 1.0).draw(gl);

        let polygon = test_polygons::ALL[self.test_polygon_index];
        let polygon = conversion::to_vec_vector2d(polygon.data);
        let triangles = triangulation_lib::process(polygon.as_slice());
        if triangles == None { return; }

        let triangles = triangles.unwrap();
        let triangles = conversion::to_vec_f64(triangles.as_slice());
        let n = triangles.len() / (3 * 2);
        let colors = test_colors::ALL;

        for i in range(0, n) {
            let start = i * 3 * 2;
            let end = (i + 1) * 3 * 2;
            c.polygon(triangles.slice(start, end))
            .color(colors[i % colors.len()]).draw(gl);
        }
    }

    fn key_press(&mut self, key: keyboard::Key) {
        if key == keyboard::Up {
            self.switch_test_polygon(1);
        } else if key == keyboard::Down {
            self.switch_test_polygon(-1);
        }
    }

    fn switch_test_polygon(&mut self, off: int) {
        self.test_polygon_index = offset(
            test_polygons::ALL.len(),
            self.test_polygon_index,
            off
        );

        // TEST
        println!("test_polygon_index {}", self.test_polygon_index);
    }
}

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Rust-Graphics-Lab: Triangulation App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut app = App::new(opengl);

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        use event::{ PressEvent, RenderEvent };
        e.render(|args| {
            app.draw(args);
        });
        e.press(|button| {
            match button {
                input::Keyboard(key) => {
                    app.key_press(key);
                }
                _ => {}
            }
        });
    }
}

