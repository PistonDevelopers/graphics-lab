
// Allow dead code since this is an example app.
#![allow(dead_code)]
#![feature(globs)]

extern crate shader_version;
extern crate event;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{Gl};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use event::{
    EventIterator,
    EventSettings,
    WindowSettings,
};

fn render(c: &Context, gl: &mut Gl) { 
    // c.ellipse(0.0, 0.0, 100.0, 200.0).rgb(1.0, 0.0, 0.0).border_width(3.0).draw(gl);

    // c.rect(0.0, 0.0, 100.0, 100.0).round(20.0).rgb(1.0, 0.0, 0.0).border_width(3.0).draw(gl);

    // c.rect(0.0, 0.0, 100.0, 100.0).bevel(20.0).rgb(1.0, 0.0, 0.0).border_width(3.0).draw(gl);

    c.rect(20.0, 20.0, 220.0, 200.0).border_width(10.0).rgb(1.0, 0.0, 0.0).draw(gl);
}

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "Rust-Graphics-Lab: Line App".to_string(),
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
    let ref mut gl = Gl::new(opengl);
    for e in event_iter {
        use event::RenderEvent;
        e.render(|args| {
            gl.viewport(0, 0, args.width as i32, args.height as i32);
            let c = Context::abs(args.width as f64, args.height as f64);
            c.rgb(1.0, 1.0, 1.0).draw(gl);
            render(&c, gl);
        });
    }
}
