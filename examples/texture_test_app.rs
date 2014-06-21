
// Allow dead code since this is an example app.
#![allow(dead_code)]

extern crate piston;
extern crate graphics;

use graphics::{
    AddImage,
    Context,
    Draw,
    RelativeTransform2d,
};
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindowSDL2,
    GameWindowSettings,
    Render,
    Texture,
};

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Rust-Graphics-Lab: Texture App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let asset_store = AssetStore::from_folder("assets");

    let image = asset_store.path("dices.png").unwrap();
    let image = Texture::from_path(&image).unwrap();

    let image2 = asset_store.path("dices.png").unwrap();
    let image2 = Texture::from_path(&image2).unwrap();

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
                    let offset = 150.0;
                    c.trans(0.0, offset).image(&image).draw(args.gl);
                    c.trans(offset, 0.0).image(&image2).draw(args.gl);
                    c.trans(offset, offset).image(&image2).draw(args.gl);
                },
                _ => {},
            }
        }
    }
}

