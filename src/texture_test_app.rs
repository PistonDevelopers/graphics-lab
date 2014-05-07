
// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;
use glfw::{Key};

pub struct App {
    image: Option<Image> 
}

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        c.view().image(self.image.unwrap()).alpha(0.2).draw(gl);
    }

    fn update(&mut self, _dt: f64, _asset_store: &mut AssetStore) {
        
    }

    fn load(&mut self, asset_store: &mut AssetStore) {
        self.image = Some(asset_store.load_image("dices.png"));
    }

    fn key_press(&mut self, _key: Key, _asset_store: &mut AssetStore) {

    }

    fn key_release(&mut self, _key: Key, _asset_store: &mut AssetStore) {

    }
}

impl App {
    pub fn new() -> App {
        App {
            image: None,
        }
    }
}

