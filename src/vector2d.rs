
//! A Vector2d struct for easier translation of algorithms.

/// Makes it easier to test translated algorithms from other languages.
#[deriving(PartialEq)]
pub struct Vector2d {
    /// The x component of the vector.
    pub x: f64,
    /// The y component of the vector.
    pub y: f64,
}

impl Vector2d {
    /// Gets x component.
    pub fn get_x(&self) -> f64 { self.x }

    /// Gets y component.
    pub fn get_y(&self) -> f64 { self.y }

    /// Sets x and y component.
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

