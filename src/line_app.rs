
// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;
use glfw::{Key};

pub struct App;

fn line(x1: f64, y1: f64, x2: f64, y2: f64, width: f64, c: &ColorContext, gl: &mut Gl) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = (dx * dx + dy * dy).sqrt();
    c.trans_local(x1, y1)
    .orient_local(x2 - x1, y2 - y1)
    .rect(0.0, -0.5 * width, length, width)
    .fill(gl);
}

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        line(0.0, 0.0, 0.5, 0.0, 0.04, &c.rgb(1.0, 0.0, 0.0).trans(0.5, 0.5), gl);
    }

    fn update(&mut self, _dt: f64, _asset_store: &mut AssetStore) {

    }

    fn load(&mut self, _asset_store: &mut AssetStore) {

    }

    fn key_press(&mut self, _key: Key, _asset_store: &mut AssetStore) {

    }

    fn key_release(&mut self, _key: Key, _asset_store: &mut AssetStore) {

    }
}

impl App {
    pub fn new() -> App {
        App
    }
}

