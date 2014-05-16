// Allow dead code since this is an example app.
#![allow(dead_code)]

// use graphics::*;
use piston::*;
use {
    GameWindowBackEnd,
};

pub struct App;

impl Game<GameWindowBackEnd> for App {
}

impl App {
    pub fn new() -> App {
        App
    }
}

