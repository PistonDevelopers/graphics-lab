
// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;
use {
    GameWindowBackEnd,
};

pub struct App {
    image: Option<Image>,
    image2: Option<Image>, 
}

impl Game<GameWindowBackEnd> for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        c.view().grey(0.5).image(self.image.unwrap()).draw(gl);
        c.view().trans(0.5, -0.5).image(self.image2.unwrap()).draw(gl);
    }

    fn load(&mut self, asset_store: &mut AssetStore) {
        self.image = Some(asset_store.load_image("dices.png"));
        self.image2 = Some(asset_store.load_image("dices.png"));
    }
}

impl App {
    pub fn new() -> App {
        App {
            image: None,
            image2: None,
        }
    }
}

