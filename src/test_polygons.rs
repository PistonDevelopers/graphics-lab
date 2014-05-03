
pub struct TestPolygon {
    pub minimum_triangles: Option<u64>,
    pub data: &'static [f64],
}

pub static ALL: &'static [TestPolygon] = &[
    SQUARE_CLOCKWISE,
    SQUARE_COUNTER_CLOCKWISE,
];

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


