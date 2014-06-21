
extern crate debug;
extern crate lab;

use lab::units::Per;
use lab::units::Comp;

pub enum L1Cache {}
#[deriving(PartialEq)]
pub enum F32 {}
#[deriving(PartialEq)]
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

impl Comp<Byte> for F32 {
    fn comp() -> Per<Byte, F32> {
        Per::new(4.0)
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

fn bytes_per_xy<T: Comp<Byte>>() -> Per<Byte, XY<T>> {
    let bytes_per_t: Per<Byte, T> = Comp::comp();
    let t_per_xy: Per<T, XY<T>> = Comp::comp();
    bytes_per_t * t_per_xy
}

fn bytes_per_xyz<T: Comp<Byte>>() -> Per<Byte, XYZ<T>> {
    let bytes_per_t: Per<Byte, T> = Comp::comp();
    let t_per_xyz: Per<T, XYZ<T>> = Comp::comp();
    bytes_per_t * t_per_xyz
}

fn main() {
    let bytes_per_xy: Per<Byte, XY<F32>> = bytes_per_xy();
    println!("{:?}", bytes_per_xy);
   
    let bytes_per_xyz: Per<Byte, XYZ<F32>> = bytes_per_xyz(); 
    println!("{:?}", bytes_per_xyz);
}

