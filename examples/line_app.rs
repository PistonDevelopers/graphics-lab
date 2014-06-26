
// Allow dead code since this is an example app.
#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{Gl};
use Window = sdl2_game_window::GameWindowSDL2;
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
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};

fn render(c: &Context, gl: &mut Gl) {
    let line = c.line(0.0, 0.0, 0.5, 0.5);
    let line = line.round_border_radius(0.1);
    let line = line.rgb(1.0, 1.0, 0.0);
    let n = 200;
    let (start, end) = (-0.75, 0.5);
    for i in range(0, n + 1) {
        let f = i as f64 / n as f64;
        line.trans(f * (end - start) + start, 0.0)
        .hue_deg(f as f32 * 360.0)
        .draw(gl);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Rust-Graphics-Lab: Line App".to_string(),
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
            None => break,
            Some(mut e) => match e {
                Render(ref mut args) => {
                    gl.viewport(0, 0, args.width as i32, args.height as i32);
                    let c = Context::abs(
                            args.width as f64, 
                            args.height as f64
                        );
                    c.rgb(1.0, 1.0, 1.0).draw(gl);
                    render(&c, gl);
                },
                _ => {},
            },
        }
    }
}
