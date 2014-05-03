
// Extern crates.
use piston::*;
use graphics::*;

// Local crate.
use test_polygons;
use conversion;
use snipcode_triangulation;

pub struct App;

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        let polygon = test_polygons::SQUARE_CLOCKWISE;
        let polygon = conversion::to_vec_vector2d(polygon.data);
        let triangles = snipcode_triangulation::process(polygon.as_slice());
    }
}

impl App {
    pub fn new() -> App {
        App
    }
}


