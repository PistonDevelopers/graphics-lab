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
    u: uint,
    v: uint,
    w: uint,
    n: uint,
    V: &mut Vec<uint>
) -> bool {

    let Ax = contour.get(*V.get(u)).unwrap().get_x();
    let Ay = contour.get(*V.get(u)).unwrap().get_y();

    let Bx = contour.get(*V.get(v)).unwrap().get_x();
    let By = contour.get(*V.get(v)).unwrap().get_y();

    let Cx = contour.get(*V.get(w)).unwrap().get_x();
    let Cy = contour.get(*V.get(w)).unwrap().get_y();

    if EPSILON > (((Bx-Ax)*(Cy-Ay)) - ((By-Ay)*(Cx-Ax))) {
        return false;
    }

    for p in range(0, n) {
        if (p == u) || (p == v) || (p == w) { continue; }

        let Px = contour.get(*V.get(p)).unwrap().get_x();
        let Py = contour.get(*V.get(p)).unwrap().get_y();

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
    let mut V: Vec<uint> = Vec::with_capacity(n);

    /* we want a counter-clockwise polygon in V */

    // Computes the area which has opposite signs
    // whether the polygon is clock-wise or counter-clockwise.
    if 0.0_f64 < area(contour) {
        for v in range(0, n) { V.push(v); }
    } else {
        for v in range(0, n) { V.push((n-1)-v); }
    }

    let mut number_of_vertices = n;

    /*  remove nv-2 Vertices, creating 1 triangle every time */
    let mut count = 2 * number_of_vertices;   /* error detection */

    let mut m = 0;
    let mut v = number_of_vertices - 1;
    while number_of_vertices > 2 {
        /* if we loop, it is probably a non-simple polygon */
        if count <= 0 {
            //** Triangulate: ERROR - probable bad polygon!
            return None;
        }

        count -= 1;

        /* three consecutive vertices in current polygon, <u,v,w> */
        let u = v;
        v = vecmath::modular_offset_index(number_of_vertices, v, 1);
        let w = vecmath::modular_offset_index(number_of_vertices, v, 1);

        if snip(contour, u, v, w, number_of_vertices, &mut V) {
            for &i in [u, v, w].iter() {
                result.push(*contour.get(*V.get(i)).unwrap());
            }

            m += 1;

            /* remove v from remaining polygon */
            for i in range(0, number_of_vertices - 3) {
                let s = v + i;
                let t = v + i + 1;

                *V.get_mut(s) = *V.get(t);
            }

            // "Removed" a vertex.
            number_of_vertices -= 1;

            /* resest error detection counter */
            count = 2 * number_of_vertices;
        }
    }

    Some(result)
}

