
/// Makes it easier to test translated algorithms from other languages.
pub struct Vector2d {
    m_x: f64,
    m_y: f64,
}

impl Vector2d {
    pub fn get_x(&self) -> f64 { self.m_x }
    pub fn get_y(&self) -> f64 { self.m_y }
    pub fn set(&mut self, x: f64, y: f64) {
        self.m_x = x;
        self.m_y = y;
    }
}
