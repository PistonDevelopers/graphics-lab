
//! Converts vector data.

// Local crate.
use vector2d::Vector2d as Vector2d;

/// Converts to the format used by Rust-Graphics.
pub fn to_vec_f64(vec: &[Vector2d]) -> Vec<[f64, ..2]> {
    let n = vec.len();
    let mut res = Vec::with_capacity(n);
    for v in vec.iter() {
        res.push([v.get_x(), v.get_y()]);
    }

    res
}

/// Converts to format used by many graphics algorithms.
pub fn to_vec_vector2d(vec: &[[f64, ..2]]) -> Vec<Vector2d> {
    let n = vec.len();
    let mut res = Vec::with_capacity(n);
    for i in range(0, n) {
        let p = vec[i];
        res.push(Vector2d {
            x: p[0],
            y: p[1],
        });
    }

    res
}

