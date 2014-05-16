#![allow(dead_code)]

// Extern crates.
use piston::*;
use graphics::*;
use graphics::modular_index::{offset};

// Local crate.
use test_colors;
use test_polygons;
use conversion;
use snipcode_triangulation;

pub struct App {
    test_polygon_index: uint,
}

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        let polygon = test_polygons::ALL[self.test_polygon_index];
        let polygon = conversion::to_vec_vector2d(polygon.data);
        let triangles = snipcode_triangulation::process(polygon.as_slice());
        if triangles == None { return; }

        let triangles = triangles.unwrap();
        let triangles = conversion::to_vec_f64(triangles.as_slice());
        let n = triangles.len() / (3 * 2);
        let colors = test_colors::ALL;

        for i in range(0, n) {
            let start = i * 3 * 2;
            let end = (i + 1) * 3 * 2;
            c.polygon(triangles.slice(start, end))
            .color(colors[i % colors.len()]).fill(gl);
        }
    }

    fn key_press(&mut self, key: keyboard::Key, _asset_store: &mut AssetStore) {
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


