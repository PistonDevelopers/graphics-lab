
use piston::*;
use graphics::*;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }
}

impl<T: GameWindow> Game<T> for App {
    fn mouse_move(&mut self, x: f64, y: f64, _asset_store: &mut AssetStore) {
        println!("{} {}", x, y);
    }

    fn key_press(&mut self, key: keyboard::Key, _asset_store: &mut AssetStore) {
        println!("Pressed {:?}", key);
    }
}
