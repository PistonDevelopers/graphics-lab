
use piston::*;
use graphics::*;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }
}

impl Game for App {
    fn mouse_move(&mut self, args: &MouseMoveArgs) {
        println!("{} {}", args.x, args.y);
    }

    fn key_press(&mut self, args: &KeyPressArgs) {
        println!("Pressed {:?}", args.key);
    }
}
