use graphics::*;

// Local crate.
use Vector2d = vector2d::Vector2d;

static EPSILON: f64 = 0.0000000001;

pub fn area(contour: &[Vector2d]) -> f64 {
    let n = contour.len();
    let mut A = 0.0_f64;

    for i in range(0, n) {
        let q = i;
        let p = (i + n - 1) % n;
        A += contour.get(p).unwrap().get_x() * contour.get(q).unwrap().get_y() 
            - contour.get(q).unwrap().get_x() * contour.get(p).unwrap().get_y();
    }

    A * 0.5_f64
}

/// InsideTriangle decides if a point P is Inside of the triangle
/// defined by A, B, C.
pub fn inside_triangle(
    Ax: f64, Ay: f64,
    Bx: f64, By: f64,
    Cx: f64, Cy: f64,
    Px: f64, Py: f64
) -> bool {
    
    let ax = Cx - Bx;
    let ay = Cy - By;
    let bx = Ax - Cx;
    let by = Ay - Cy;
    let cx = Bx - Ax;
    let cy = By - Ay;
    let apx = Px - Ax;
    let apy = Py - Ay;
    let bpx = Px - Bx;
    let bpy = Py - By;
    let cpx = Px - Cx;
    let cpy = Py - Cy;

    let aCROSSbp = ax*bpy - ay*bpx;
    let cCROSSap = cx*apy - cy*apx;
    let bCROSScp = bx*cpy - by*cpx;

    (
        (aCROSSbp >= 0.0_f64) 
        && (bCROSScp >= 0.0_f64) 
        && (cCROSSap >= 0.0_f64)
    )
}

pub fn snip(
    contour: &[Vector2d],
    triangle: [uint, ..3],
    n: uint,
    vertex_indices: &Vec<uint>
) -> bool {
    let u = triangle[0];
    let v = triangle[1];
    let w = triangle[2];

    let A = contour.get(*vertex_indices.get(u)).unwrap();
    let B = contour.get(*vertex_indices.get(v)).unwrap();
    let C = contour.get(*vertex_indices.get(w)).unwrap();
    
    let (Ax, Ay) = (A.get_x(), A.get_y());
    let (Bx, By) = (B.get_x(), B.get_y());
    let (Cx, Cy) = (C.get_x(), C.get_y());

    if EPSILON > (((Bx-Ax)*(Cy-Ay)) - ((By-Ay)*(Cx-Ax))) {
        return false;
    }

    for p in range(0, n) {
        if (p == u) || (p == v) || (p == w) { continue; }

        let Px = contour.get(*vertex_indices.get(p)).unwrap().get_x();
        let Py = contour.get(*vertex_indices.get(p)).unwrap().get_y();

        if inside_triangle(Ax,Ay,Bx,By,Cx,Cy,Px,Py) { return false; }
    }

    return true;
}

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
            vecmath::modular_offset_index(number_of_vertices, v, 1),
            vecmath::modular_offset_index(number_of_vertices, v, 2)
        ];
        v = triangle[1];

        if snip(contour, triangle, number_of_vertices, &mut vertex_indices) {
            for &i in triangle.iter() {
                result.push(*contour.get(*vertex_indices.get(i)).unwrap());
            }

            /* remove v from remaining polygon */
            for i in range(0, number_of_vertices - 3) {
                let s = v + i;
                let t = v + i + 1;

                *vertex_indices.get_mut(s) = *vertex_indices.get(t);
            }

            // "Removed" a vertex.
            number_of_vertices -= 1;

            /* resest error detection counter */
            count = 2 * number_of_vertices;
        }
    }

    Some(result)
}

