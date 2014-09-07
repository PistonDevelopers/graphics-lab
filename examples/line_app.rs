
// Allow dead code since this is an example app.
#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{Gl};
use sdl2_game_window::WindowSDL2;
use graphics::{
    Context,
    AddColor,
    AddLine,
    AddRoundBorder,
    RelativeColor,
    RelativeTransform2d,
    Draw,
};
use piston::{
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
};

fn render(c: &Context, gl: &mut Gl) {
    // Create a line.
    let line = c.line(0.0, 0.0, 0.0, 100.0)
        .round_border_radius(3.0)
        .rgb(1.0, 1.0, 0.0);

    // Draw ten lines beside each other with hue transformed color.
    let n = 10;
    let (start, end) = (0.0, 400.0);
    for i in range(0u, n + 1) {
        let f = i as f64 / n as f64;
        line.trans(f * (end - start) + start, 0.0)
        .hue_deg(f as f32 * 360.0)
        .draw(gl);
    }
}

fn main() {
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
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
        match e {
            Render(ref args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                render(&c, gl);
            }
            _ => {}
        }
    }
}
