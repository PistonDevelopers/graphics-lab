
// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;

pub struct App {
    image: Option<Image>,
    image2: Option<Image>, 
}

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        let offset = 150.0;
        c.trans(0.0, offset).image(self.image.unwrap()).draw(gl);
        c.trans(offset, 0.0).image(self.image2.unwrap()).draw(gl);
        c.trans(offset, offset).image(self.image2.unwrap()).draw(gl);
    }

    fn load(&mut self, asset_store: &mut AssetStore) {
        self.image = Some(asset_store.load_image("dices.png").unwrap());
        self.image2 = Some(asset_store.load_image("dices.png").unwrap());
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

