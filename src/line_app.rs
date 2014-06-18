
// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;

pub struct App;

impl Game for App {
    fn render(&self, c: &Context, args: &mut RenderArgs) {
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
}

impl App {
    pub fn new() -> App {
        App
    }
}

