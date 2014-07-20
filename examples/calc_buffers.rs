
extern crate debug;
extern crate lab = "rust-graphics-lab";

use lab::units::Per;
use lab::units::Comp;

pub enum Sum<A, B> {}
pub enum L1Cache {}
pub enum F32 {}
pub enum F64 {}
pub enum Byte {}
pub enum XY<T> {}
pub enum XYZ<T> {}
pub enum RGB<T> {}
pub enum RGBA<T> {}
pub enum Triangle<T> {}
pub enum Quad<T> {}

impl Comp<Byte> for L1Cache {
    fn comp() -> Per<Byte, L1Cache> {
        Per::kibi(32.0)
    }
}

impl<T> Comp<XY<T>> for Triangle<XY<T>> {
    fn comp() -> Per<XY<T>, Triangle<XY<T>>> {
        Per::new(3.0)
    }
}

impl<T> Comp<XYZ<T>> for Triangle<XYZ<T>> {
    fn comp() -> Per<XYZ<T>, Triangle<XYZ<T>>> {
        Per::new(3.0)
    }
}

impl<T> Comp<Triangle<T>> for Quad<Triangle<T>> {
    fn comp() -> Per<Triangle<T>, Quad<Triangle<T>>> {
        Per::new(2.0)
    }
}

impl<T> Comp<XY<T>> for Quad<XY<T>> {
    fn comp() -> Per<XY<T>, Quad<XY<T>>> {
        Per::new(4.0)
    }
}

impl<T> Comp<XYZ<T>> for Quad<XYZ<T>> {
    fn comp() -> Per<XYZ<T>, Quad<XYZ<T>>> {
        Per::new(4.0)
    }
}

impl Comp<Byte> for Byte {
    fn comp() -> Per<Byte, Byte> {
        Per::new(1.0)
    }
}

impl Comp<Byte> for F32 {
    fn comp() -> Per<Byte, F32> {
        Per::new(4.0)
    }
}

impl Comp<Byte> for F64 {
    fn comp() -> Per<Byte, F64> {
        Per::new(8.0)
    }
}

impl<T> Comp<T> for XY<T> {
    fn comp() -> Per<T, XY<T>> {
        Per::new(2.0)
    }
}

impl<T> Comp<T> for XYZ<T> {
    fn comp() -> Per<T, XYZ<T>> {
        Per::new(3.0)
    }
}

impl<T> Comp<T> for RGB<T> {
    fn comp() -> Per<T, RGB<T>> {
        Per::new(3.0)
    }
}

impl<T> Comp<T> for RGBA<T> {
    fn comp() -> Per<T, RGBA<T>> {
        Per::new(4.0)
    }
}

fn product<T: Comp<U>, U, V: Comp<T>>() -> Per<U, V> {
    let u_per_t: Per<U, T> = Comp::comp();
    let t_per_v: Per<T, V> = Comp::comp();
    u_per_t * t_per_v
}

impl<T: Comp<Byte>, U: Comp<T>, V: Comp<Byte>, W: Comp<V>> 
Comp<Byte> for Sum<U, W> {
    fn comp() -> Per<Byte, Sum<U, W>> {
        let Per(a): Per<Byte, W> = product();
        let Per(b): Per<Byte, W> = product();
        Per(a + b)
    }
}

fn bytes_per_triangle<T: Comp<Byte>>() -> Per<Byte, Triangle<XY<T>>> { 
    let bytes_per_xy: Per<Byte, XY<T>> = product();
    bytes_per_xy * Comp::comp()
}

fn bytes_per_xy<T: Comp<Byte>>() -> Per<Byte, XY<T>> {
    product()
}

fn bytes_per_xyz<T: Comp<Byte>>() -> Per<Byte, XYZ<T>> {
    product()
}

fn bytes_per_l1cache() -> Per<Byte, L1Cache> {
    Comp::comp()
}

fn bytes_per_rgba<T: Comp<Byte>>() -> Per<Byte, RGBA<T>> {
    product()
}

fn bytes_per_rgb<T: Comp<Byte>>() -> Per<Byte, RGB<T>> {
    product()
}

fn main() {
    let bytes_per_xy: Per<Byte, XY<F32>> = bytes_per_xy();
    println!("{:?}", bytes_per_xy);
   
    let bytes_per_xyz: Per<Byte, XYZ<F32>> = bytes_per_xyz();
    println!("{:?}", bytes_per_xyz);

    let bytes_per_triangle: Per<Byte, Triangle<XY<F32>>>
        = bytes_per_triangle();
    println!("{:?}", bytes_per_triangle);

    let bytes_per_l1cache: Per<Byte, L1Cache> = bytes_per_l1cache();
    println!("{:?}", bytes_per_l1cache);

    let triangles_per_l1cache = bytes_per_l1cache / bytes_per_triangle;
    println!("{:?}", triangles_per_l1cache);

    let bytes_per_rgba: Per<Byte, RGBA<F32>> = bytes_per_rgba();
    println!("{:?}", bytes_per_rgba);

    let bytes_per_rgb: Per<Byte, RGB<F32>> = bytes_per_rgb();
    println!("{:?}", bytes_per_rgb);
}

