// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;
use glfw::{Key};

pub struct App;

impl Game for App {
    fn render(&self, _c: &Context, _gl: &mut Gl) {

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

