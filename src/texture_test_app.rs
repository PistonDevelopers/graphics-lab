
// Allow dead code since this is an example app.
#![allow(dead_code)]

use graphics::*;
use piston::*;

pub struct App {
    image: Option<Texture>,
    image2: Option<Texture>, 
}

impl Game for App {
    fn render(&self, c: &Context, args: &mut RenderArgs) {
        let offset = 150.0;
        c.trans(0.0, offset).image(self.image.as_ref().unwrap()).draw(args.gl);
        c.trans(offset, 0.0).image(self.image2.as_ref().unwrap()).draw(args.gl);
        c.trans(offset, offset).image(self.image2.as_ref().unwrap()).draw(args.gl);
    }

    fn load(&mut self, asset_store: &mut AssetStore) {
        let image = asset_store.path("dices.png").unwrap();
        self.image = Some(Texture::from_path(&image).unwrap());

        let image = asset_store.path("dices.png").unwrap();
        self.image2 = Some(Texture::from_path(&image).unwrap());
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

