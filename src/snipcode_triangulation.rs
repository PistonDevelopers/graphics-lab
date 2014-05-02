static EPSILON: f64 = 0.0000000001;

pub struct Vector2d {
    m_x: f64,
    m_y: f64,
}

impl Vector2d {
    fn get_x(&self) -> f64 { self.m_x }
    fn get_y(&self) -> f64 { self.m_y }
    fn set(&mut self, x: f64, y: f64) {
        self.m_x = x;
        self.m_y = y;
    }
}

fn area(contour: &Vec<Vector2d>) -> f64 {
    let n = contour.len();

    let mut A = 0.0_f64;

    for i in range(0, n) {
        let q = i;
        let p = (n + i - 1) % n;
        A += contour.get(p).get_x() * contour.get(q).get_y() 
            - contour.get(q).get_x() * contour.get(p).get_y();
    }
    
    A * 0.5_f64
}

/// InsideTriangle decides if a point P is Inside of the triangle
/// defined by A, B, C.
fn inside_triangle(
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

fn snip(
    contour: &Vec<Vector2d>,
    u: uint,
    v: uint,
    w: uint,
    n: uint,
    V: &mut Vec<uint>
) -> bool {

    let Ax = contour.get(*V.get(u)).get_x();
    let Ay = contour.get(*V.get(u)).get_y();

    let Bx = contour.get(*V.get(v)).get_x();
    let By = contour.get(*V.get(v)).get_y();

    let Cx = contour.get(*V.get(w)).get_x();
    let Cy = contour.get(*V.get(w)).get_y();

    if EPSILON > (((Bx-Ax)*(Cy-Ay)) - ((By-Ay)*(Cx-Ax))) {
        return false;
    }

    for p in range(0, n) {
        if (p == u) || (p == v) || (p == w) { continue; }

        let Px = contour.get(*V.get(p)).get_x();
        let Py = contour.get(*V.get(p)).get_y();

        if inside_triangle(Ax,Ay,Bx,By,Cx,Cy,Px,Py) { return false; }
    }

    return true;
}

fn process(
    contour: &Vec<Vector2d>
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
        for v in range(0, n) { *V.get_mut(v) = v; }
    } else {
        for v in range(0, n) { *V.get_mut(v) = (n-1)-v; }
    }

    let mut nv = n;

    /*  remove nv-2 Vertices, creating 1 triangle every time */
    let mut count = 2*nv;   /* error detection */

    let mut m = 0;
    let mut v = nv-1;
    while nv>2 {
        /* if we loop, it is probably a non-simple polygon */
        if (0 >= count) {
            count -= 1;

            //** Triangulate: ERROR - probable bad polygon!
            return None;
        }

        count -= 1;

        /* three consecutive vertices in current polygon, <u,v,w> */
        let mut u = v  ;
        if nv <= u { u = 0; }     /* previous */
        v = u+1;
        if nv <= v { v = 0; }    /* new v    */
        let mut w = v+1;
        if nv <= w { w = 0; }     /* next     */

        if snip(contour, u, v, w, nv, &mut V) {
            /* true names of the vertices */
            let a = *V.get(u);
            let b = *V.get(v);
            let c = *V.get(w);

            /* output Triangle */
            result.push(*contour.get(a));
            result.push(*contour.get(b));
            result.push(*contour.get(c));

            m += 1;

            /* remove v from remaining polygon */
            for i in range(0, nv-1) {
                let s = v + i;
                let t = v + i + 1;
                *V.get_mut(s) = *V.get(t);
            }

            // Perhaps this should be added to the for-loop above?
            nv -= 1;

            /* resest error detection counter */
            count = 2*nv;
        }
    }

    Some(result)
}

