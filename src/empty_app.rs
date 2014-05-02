
use piston;
use graphics;
use graphics::*;
use piston::game::*;
use piston::gl::{Gl};
use glfw::{Key};

pub struct EmptyApp {
    settings: Settings,
}

impl Game for EmptyApp {
    fn get_settings<'a>(&'a self) -> &'a Settings { &self.settings }

    fn render(&self, c: &Context, gl: &mut Gl) {

    }

    fn update(&mut self, dt: f64) {

    }

    fn load(&mut self) {

    }

    fn key_press(&mut self, key: Key) {

    }

    fn key_release(&mut self, key: Key) {

    }
}

impl EmptyApp {
    pub fn new(settings: Settings) -> EmptyApp {
        EmptyApp {
            settings: settings
        }
    }
}
