
// Allow dead code since this is an example app.
#![allow(dead_code)]
#![feature(globs)]

extern crate shader_version;
extern crate event;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{Gl, Texture};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use event::{
    EventIterator,
    EventSettings,
    WindowSettings,
};

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
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

    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut game_iter = EventIterator::new(&mut window, &event_settings);
    let ref mut gl = Gl::new(opengl);
    for e in game_iter {
        use event::RenderEvent;
        e.render(|args| {
            gl.viewport(0, 0, args.width as i32, args.height as i32);
            let c = Context::abs(args.width as f64, args.height as f64);
            c.rgb(1.0, 1.0, 1.0).draw(gl);
            c.rect(0.0, 0.0, 50.0, 50.0).rgb(1.0, 0.0, 0.0).draw(gl);
            let offset = 150.0;
            c.trans(0.0, offset).image(&image).draw(gl);
        });
    }
}

