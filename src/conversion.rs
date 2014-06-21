
//! Converts vector data.

// Local crate.
use Vector2d = vector2d::Vector2d;

/// Converts to the format used by Rust-Graphics.
pub fn to_vec_f64(vec: &[Vector2d]) -> Vec<f64> {
    let n = vec.len();
    let mut res = Vec::with_capacity(n * 2);
    for v in vec.iter() {
        res.push(v.get_x());
        res.push(v.get_y());
    }

    res
}

/// Converts to format used by many graphics algorithms.
pub fn to_vec_vector2d(vec: &[f64]) -> Vec<Vector2d> {
    let n = vec.len() / 2;
    let mut res = Vec::with_capacity(n);
    for i in range(0, n) {
        res.push(Vector2d {
            x: *vec.get(i * 2).unwrap(),
            y: *vec.get(i * 2 + 1).unwrap(),
        });
    }

    res
}

