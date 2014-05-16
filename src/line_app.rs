
// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;

pub struct App;

fn return_test(c: &Context) -> BevelRectangleContext {
    c.rect(0.0, 0.0, 0.5, 0.5).bevel(0.1).clone()
}    

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        c.line(0.0, 0.0, 0.5, 0.5).square_border_radius(0.1).rgb(1.0, 0.0, 0.0).stroke(gl);
        match return_test(c) {
            c => c.rgb(0.0, 1.0, 0.0).fill(gl)
        };
    }
}

impl App {
    pub fn new() -> App {
        App
    }
}

