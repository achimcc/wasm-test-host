#![no_std]

#[macro_use]
extern crate ark_std;

mod boundary;
pub use boundary::{Boundary, CallId, NativeBoundary};
