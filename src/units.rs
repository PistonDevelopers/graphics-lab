
#![allow(missing_doc)]

//! Provides safe unit abstractions.

pub trait Comp<T> {
    fn comp() -> Per<T, Self>;
}

#[deriving(PartialEq)]
pub struct Per<A, B>(pub f64);

impl<T, U> Per<T, U> {
    pub fn new(num: f64) -> Per<T, U> {
        Per(num)
    }

    pub fn kibi(num: f64) -> Per<T, U> {
        Per(num * 1024.0)
    }
}

impl<T, U, V> Div<Per<T, V>, Per<V, U>> for Per<T, U> {
    fn div(&self, rhs: &Per<T, V>) -> Per<V, U> {
        match (*self, *rhs) {
            (Per(a), Per(b)) => Per(a / b),
        }
    }
}

impl<T, U, V> Mul<Per<U, V>, Per<T, V>> for Per<T, U> {
    fn mul(&self, rhs: &Per<U, V>) -> Per<T, V> {
        match (*self, *rhs) {
            (Per(a), Per(b)) => Per(a * b),
        }
    }
}

impl<T, U> Add<Per<T, U>, Per<T, U>> for Per<T, U> {
    fn add(&self, rhs: &Per<T, U>) -> Per<T, U> {
        match (*self, *rhs) {
            (Per(a), Per(b)) => Per(a + b),
        }
    }
}

impl<T, U> Sub<Per<T, U>, Per<T, U>> for Per<T, U> {
    fn sub(&self, rhs: &Per<T, U>) -> Per<T, U> {
        match (*self, *rhs) {
            (Per(a), Per(b)) => Per(a - b),
        }
    }
}

