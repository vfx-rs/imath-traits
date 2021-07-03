//! imath-traits provides a set of traits which constrain the types used in Rust translations of
//! C++ APIs that rely on `Imath`, or `Imath-alike` types.
//!
//! This is solely about memory layout and being able to convert the implementing types back and
//! forward into slices and pointers to be able to be used in the FFI call, thus the traits contain
//! no methods other than for converting back and forth between slices and raw pointers.
//!
//! To use, simply add the feature for the math crate you need to the dependency
//! of any crate that uses imath-traits (these will be called `impl_<crate>`, and types will just work with any function
//! from that crate that expects a Vec2<T>, Vec3<T>, Vec4<T>, Bound2<T> or Bound3<T>:
//!
//! ```toml
//! openexr = { version = "0.10-3.0.1", features=["impl_cgmath"] }
//! ```
//!
//! Currently, we support glam, nalgebra and nalgebra_glm. If you need another math
//! crate, implement support for it and submit a PR, or request it. Note that the
//! crate must support 2-, 3- and 4-dimensional vectors of i32, f32 and f64.
//!

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
#[cfg(feature = "cgmath")]
pub use impl_cgmath::{Box2, Box3};

#[cfg(feature = "glam")]
pub mod impl_glam;
#[cfg(feature = "glam")]
pub use impl_glam::{Box2, Box3};

#[cfg(feature = "nalgebra")]
pub mod impl_nalgebra;
#[cfg(feature = "nalgebra")]
pub use impl_nalgebra::{Box2, Box3};

#[cfg(feature = "nalgebra-glm")]
pub mod impl_nalgebra_glm;
#[cfg(feature = "nalgebra_glm")]
pub use impl_nalgebra_glm::{Box2, Box3};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
