// Allow dead code since this is an example app.
#![allow(dead_code)]

// use graphics::*;
use piston::*;

pub struct App;

impl<T: GameWindow> Game<T> for App {
}

impl App {
    pub fn new() -> App {
        App
    }
}

