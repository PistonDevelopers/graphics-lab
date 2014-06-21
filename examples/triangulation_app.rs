#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate lab;

use graphics::{
    AddColor,
    AddPolygon,
    Context,
    Fill,
    View,
};
use piston::{
    AssetStore,
    Game,
    GameWindowSDL2,
    GameWindowSettings,
    keyboard,
    KeyPressArgs,
    RenderArgs,
};
use graphics::modular_index::{offset};

// Local crate.
use lab::test_colors;
use lab::test_polygons;
use lab::conversion;
use lab::triangulation;

pub struct App {
    test_polygon_index: uint,
}

impl Game for App {
    fn render(&self, c: &Context, args: &mut RenderArgs) {
        let c = &c.reset();

        let polygon = test_polygons::ALL[self.test_polygon_index];
        let polygon = conversion::to_vec_vector2d(polygon.data);
        let triangles = triangulation::process(polygon.as_slice());
        if triangles == None { return; }

        let triangles = triangles.unwrap();
        let triangles = conversion::to_vec_f64(triangles.as_slice());
        let n = triangles.len() / (3 * 2);
        let colors = test_colors::ALL;

        for i in range(0, n) {
            let start = i * 3 * 2;
            let end = (i + 1) * 3 * 2;
            c.polygon(triangles.slice(start, end))
            .color(colors[i % colors.len()]).fill(args.gl);
        }
    }

    fn key_press(&mut self, args: &KeyPressArgs) {
        let key = args.key;
        if key == keyboard::Up {
            self.switch_test_polygon(1);
        } else if key == keyboard::Down {
            self.switch_test_polygon(-1);
        }
    }

    
}

impl App {
    pub fn new() -> App {
        App {
            test_polygon_index: 0,
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

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run on main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Rust-Graphics-Lab: Triangulation App".to_string(),
            size: [600, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0]
        }
    );
   
    let mut asset_store = AssetStore::from_folder("assets"); 
    let mut app = App::new();
    app.run(&mut window, &mut asset_store);
}
