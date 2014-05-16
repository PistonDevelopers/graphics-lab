
use piston::*;
use graphics::*;
use {
    GameWindowBackEnd,
};

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }
}

impl Game<GameWindowBackEnd> for App {
    fn mouse_move(&mut self, x: f64, y: f64, _asset_store: &mut AssetStore) {
        println!("{} {}", x, y);
    }

    fn key_press(&mut self, key: keyboard::Key, _asset_store: &mut AssetStore) {
        println!("Pressed {:?}", key);
    }
}
