
// Allow dead code since this is an example app.
#![allow(dead_code)]

extern crate piston;
extern crate graphics;

use graphics::{
    Context,
    AddColor,
    AddLine,
    AddRoundBorder,
    RelativeColor,
    RelativeTransform2d,
    Stroke,
};
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSDL2,
    GameWindowSettings,
    Render,
    RenderArgs,
};

fn render(c: &Context, args: &mut RenderArgs) {
    let line = c.line(0.0, 0.0, 0.5, 0.5);
    let line = line.round_border_radius(0.1);
    let line = line.rgb(1.0, 1.0, 0.0);
    let n = 200;
    let (start, end) = (-0.75, 0.5);
    for i in range(0, n + 1) {
        let f = i as f64 / n as f64;
        line.trans(f * (end - start) + start, 0.0)
        .hue_deg(f as f32 * 360.0)
        .stroke(args.gl);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Rust-Graphics-Lab: Line App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0]
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
    loop {
        match game_iter.next() {
            None => break,
            Some(mut e) => match e {
                Render(ref mut args) => {
                    let c = Context::abs(
                            args.width as f64, 
                            args.height as f64
                        );
                    render(&c, args);
                },
                _ => {},
            },
        }
    }
}
