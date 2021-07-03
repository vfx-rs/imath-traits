# imath-traits

imath-traits provides a set of traits which constrain the types used in Rust translations of
C++ APIs that rely on `Imath`, or `Imath-alike` types.

This is solely about memory layout and being able to convert the implementing types back and
forward into slices and pointers to be able to be used in the FFI call, thus the traits contain
no methods other than for converting back and forth between slices and raw pointers.

To use, simply add the feature for the math crate you need to the dependency
of any crate that uses imath-traits (these will be called `imath_<crate>`, and types will just work with any function
from that crate that expects a Vec2<T>, Vec3<T>, Vec4<T>, Bound2<T> or Bound3<T>:

```toml
openexr = { version = "0.10-3.0.1", features=["imath_cgmath"] }
```

Currently, we support glam, nalgebra and nalgebra_glm. If you need another math
crate, implement support for it and submit a PR, or request it. Note that the
crate must support 2-, 3- and 4-dimensional vectors of i32, f32 and f64.


