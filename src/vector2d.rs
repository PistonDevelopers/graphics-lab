
/// Makes it easier to test translated algorithms from other languages.
#[deriving(Eq)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    pub fn get_x(&self) -> f64 { self.x }

    pub fn get_y(&self) -> f64 { self.y }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

