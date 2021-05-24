//! imath-traits provides a set of traits which constrain the types used in Rust translations of
//! C++ APIs that rely on `Imath`, or `Imath-alike` types.
//!
//! This is solely about memory layout and being able to convert the implementing types back and
//! forward into slices and pointers to be able to be used in the FFI calls.

pub use half::f16;

pub mod vec;
pub use vec::*;

pub mod bound;
pub use bound::*;

pub mod matrix;
pub use matrix::*;

pub mod zero;
pub use zero::Zero;

#[cfg(feature = "cgmath")]
pub mod impl_cgmath;

#[cfg(feature = "nalgebra")]
pub mod impl_nlagebra;

#[cfg(feature = "nalgebra_glm")]
pub mod impl_nlagebra_glm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
