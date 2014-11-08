//! Experimental triangulation algorithm.

use graphics::modular_index::{offset, previous, next};

// Local crate.
use vector2d::Vector2d as Vector2d;

static EPSILON: f64 = 0.0000000001;

/// Compute area of contour.
pub fn area(contour: &[Vector2d]) -> f64 {
    let n = contour.len();
    let mut sum = 0.0_f64;

    for i in range(0, n) {
        let q = i;
        let p = previous(n, i);
        sum += contour.get(p).unwrap().get_x() 
            * contour.get(q).unwrap().get_y() 
            - contour.get(q).unwrap().get_x() 
            * contour.get(p).unwrap().get_y();
    }

    sum * 0.5_f64
}

/// InsideTriangle decides if a point P is Inside of the triangle
/// defined by A, B, C.
pub fn inside_triangle(
    a_x: f64, a_y: f64,
    b_x: f64, b_y: f64,
    c_x: f64, c_y: f64,
    p_x: f64, p_y: f64
) -> bool {
    
    let (bcx, bcy) = (c_x - b_x, c_y - b_y);
    let (cax, cay) = (a_x - c_x, a_y - c_y);
    let (abx, aby) = (b_x - a_x, b_y - a_y);
    let (apx, apy) = (p_x - a_x, p_y - a_y);
    let (bpx, bpy) = (p_x - b_x, p_y - b_y);
    let (cpx, cpy) = (p_x - c_x, p_y - c_y);

    let bc_cross_bp = bcx * bpy - bcy * bpx;
    let ab_cross_ap = abx * apy - aby * apx;
    let ca_cross_cp = cax * cpy - cay * cpx;

    bc_cross_bp >= 0.0_f64
    && ca_cross_cp >= 0.0_f64
    && ab_cross_ap >= 0.0_f64
}

/// Check for self intersection.
pub fn snip(
    contour: &[Vector2d],
    triangle: [uint, ..3],
    n: uint,
    vertex_indices: &Vec<uint>
) -> bool {
    let (u, v, w) = (triangle[0], triangle[1], triangle[2]);

    let a_pos = contour.get(vertex_indices[u]).unwrap();
    let b_pos = contour.get(vertex_indices[v]).unwrap();
    let c_pos = contour.get(vertex_indices[w]).unwrap();
    
    let (ax, ay) = (a_pos.get_x(), a_pos.get_y());
    let (bx, by) = (b_pos.get_x(), b_pos.get_y());
    let (cx, cy) = (c_pos.get_x(), c_pos.get_y());

    if EPSILON > (((bx-ax)*(cy-ay)) - ((by-ay)*(cx-ax))) {
        return false;
    }

    for p in range(0, n) {
        // Do not check for any of the vertices in the triangle.
        if (p == u) || (p == v) || (p == w) { continue; }

        let p_pos = contour.get(vertex_indices[p]).unwrap();
        if inside_triangle(
                ax, ay,
                bx, by,
                cx, cy,
                p_pos.get_x(), p_pos.get_y()
        ) {
            return false;
        }
    }

    true
}

/// Triangulize a polygon contour.
pub fn process(
    contour: &[Vector2d]
) -> Option<Vec<Vector2d>> {
    /* allocate and initialize list of Vertices in polygon */

    let n = contour.len();
    if n < 3 { return None; }

    let mut result: Vec<Vector2d> = Vec::new();
    let mut vertex_indices: Vec<uint> = Vec::with_capacity(n);

    /* we want a counter-clockwise polygon in V */

    // Computes the area which has opposite signs
    // whether the polygon is clock-wise or counter-clockwise.
    if 0.0_f64 < area(contour) {
        for v in range(0, n) { vertex_indices.push(v); }
    } else {
        // Add indices in reverse order.
        for v in range(0, n) { vertex_indices.push((n-1)-v); }
    }

    // Stores the number of vertices.
    // Used instead of `vertex_indices.len()` to emulate removing of vertices.
    let mut number_of_vertices = n;

    /*  remove nv-2 Vertices, creating 1 triangle every time */

    // Used for error detection.
    let mut count = 2 * number_of_vertices;

    let mut v = number_of_vertices - 1;
    while number_of_vertices > 2 {
        /* if we loop, it is probably a non-simple polygon */
        if count <= 0 {
            //** Triangulate: ERROR - probable bad polygon!
            return None;
        }

        count -= 1;

        /* three consecutive vertices in current polygon, <u,v,w> */
        let triangle = [
            v,
            next(number_of_vertices, v),
            offset(number_of_vertices, v, 2)
        ];
        v = triangle[1];

        if snip(contour, triangle, number_of_vertices, &mut vertex_indices) {
            for &i in triangle.iter() {
                result.push(*contour.get(vertex_indices[i]).unwrap());
            }

            /* remove v from remaining polygon */
            for i in range(0, number_of_vertices - 3) {
                let s = v + i;
                let t = v + i + 1;

                vertex_indices[s] = vertex_indices[t];
            }

            // "Removed" a vertex.
            number_of_vertices -= 1;

            /* resest error detection counter */
            count = 2 * number_of_vertices;
        }
    }

    Some(result)
}

