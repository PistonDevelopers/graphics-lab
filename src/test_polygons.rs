//! Test polygon shapes

/// Contains test data.
pub struct TestPolygon {
    /// The theoretical minimum triangles of splitting up the contour.
    pub minimum_triangles: Option<u64>,
    /// The polygon shape.
    pub data: &'static [f64],
}

pub const ALL: &'static [TestPolygon] = &[
    CONCAVE_1,
    SELF_INTERSECTING_1,
    SQUARE_CLOCKWISE,
    SQUARE_COUNTER_CLOCKWISE,
];

pub const SELF_INTERSECTING_1: TestPolygon = TestPolygon {
    minimum_triangles: None,
    data: &[
        0.0, 0.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
    ],
};

pub const CONCAVE_1: TestPolygon = TestPolygon {
    minimum_triangles: Some(2),
    data: &[
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.5, 0.5,
        0.0, 1.0,
    ],
};

pub const SQUARE_COUNTER_CLOCKWISE: TestPolygon = TestPolygon {
    minimum_triangles: Some(2),
    data: &[
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
    ],
};

pub const SQUARE_CLOCKWISE: TestPolygon = TestPolygon {
    minimum_triangles: Some(2),
    data: &[
        0.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
    ],
};


