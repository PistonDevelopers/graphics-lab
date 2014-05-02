
// Local crate.
use Vector2d = vector2d::Vector2d;

/// Converts to the format used by Rust-Graphics.
pub fn to_vec_f64(vec: &Vec<Vector2d>) -> Vec<f64> {
        let n = vec.len();
        let mut res = Vec::with_capacity(n * 2);
        for v in vec.iter() {
            res.push(v.get_x());
            res.push(v.get_y());
        }

        res
    }
