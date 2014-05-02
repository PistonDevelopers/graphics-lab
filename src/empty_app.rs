
use piston;
use graphics;
use graphics::*;
use piston::*;
use glfw::{Key};

pub struct EmptyApp;

impl Game for EmptyApp {
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
    pub fn new() -> EmptyApp {
        EmptyApp
    }
}

