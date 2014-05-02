
pub struct TestPolygon {
    minimum_triangles: Option<u64>,
    data: &'static [f64],
}

pub static SQUARE_COUNTER_CLOCKWISE: TestPolygon = TestPolygon {
    minimum_triangles: Some(2),
    data: &[
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
    ],
};

pub static SQUARE_CLOCKWISE: TestPolygon = TestPolygon {
    minimum_triangles: Some(2),
    data: &[
        0.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
    ],
};


