//! Implements all traits for glam
//!
//! Impls `Vec2`, `Vec3`, `Vec3A`, `Vec4`
//! Defines `Box2<T>` and `Box3<T>` in terms of `Vec2` and `Vec3`, respectively.

pub use crate::*;

impl Vec2<f32> for glam::Vec2 {
    fn from_slice(slice: &[f32; 2]) -> Self {
        glam::Vec2::new(slice[0], slice[1])
    }

    fn as_slice(&self) -> &[f32; 2] {
        unsafe { &*(self.as_ptr() as *const [f32; 2]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const glam::Vec2 as *const f32
    }
}

impl Zero for glam::Vec2 {
    fn zero() -> Self {
        glam::Vec2::ZERO
    }
}

impl Vec2<f64> for glam::DVec2 {
    fn from_slice(slice: &[f64; 2]) -> Self {
        glam::DVec2::new(slice[0], slice[1])
    }

    fn as_slice(&self) -> &[f64; 2] {
        unsafe { &*(self.as_ptr() as *const [f64; 2]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const glam::DVec2 as *const f64
    }
}

impl Zero for glam::DVec2 {
    fn zero() -> Self {
        glam::DVec2::ZERO
    }
}

impl Vec2<i32> for glam::IVec2 {
    fn from_slice(slice: &[i32; 2]) -> Self {
        glam::IVec2::new(slice[0], slice[1])
    }

    fn as_slice(&self) -> &[i32; 2] {
        unsafe { &*(self.as_ptr() as *const [i32; 2]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const glam::IVec2 as *const i32
    }
}

impl Zero for glam::IVec2 {
    fn zero() -> Self {
        glam::IVec2::ZERO
    }
}

impl Vec3<f32> for glam::Vec3 {
    fn from_slice(slice: &[f32; 3]) -> Self {
        glam::Vec3::new(slice[0], slice[1], slice[2])
    }

    fn as_slice(&self) -> &[f32; 3] {
        unsafe { &*(self.as_ptr() as *const [f32; 3]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const glam::Vec3 as *const f32
    }
}

impl Zero for glam::Vec3 {
    fn zero() -> Self {
        glam::Vec3::ZERO
    }
}

impl Vec3<f64> for glam::DVec3 {
    fn from_slice(slice: &[f64; 3]) -> Self {
        glam::DVec3::new(slice[0], slice[1], slice[2])
    }

    fn as_slice(&self) -> &[f64; 3] {
        unsafe { &*(self.as_ptr() as *const [f64; 3]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const glam::DVec3 as *const f64
    }
}

impl Zero for glam::DVec3 {
    fn zero() -> Self {
        glam::DVec3::ZERO
    }
}

impl Vec3<i32> for glam::IVec3 {
    fn from_slice(slice: &[i32; 3]) -> Self {
        glam::IVec3::new(slice[0], slice[1], slice[2])
    }

    fn as_slice(&self) -> &[i32; 3] {
        unsafe { &*(self.as_ptr() as *const [i32; 3]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const glam::IVec3 as *const i32
    }
}

impl Zero for glam::IVec3 {
    fn zero() -> Self {
        glam::IVec3::ZERO
    }
}

impl Vec3<f32> for glam::Vec3A {
    fn from_slice(slice: &[f32; 3]) -> Self {
        glam::Vec3A::new(slice[0], slice[1], slice[2])
    }

    fn as_slice(&self) -> &[f32; 3] {
        unsafe { &*(self.as_ptr() as *const [f32; 3]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const glam::Vec3A as *const f32
    }
}

impl Zero for glam::Vec3A {
    fn zero() -> Self {
        glam::Vec3A::ZERO
    }
}

impl Vec4<f32> for glam::Vec4 {
    fn from_slice(slice: &[f32; 4]) -> Self {
        glam::Vec4::new(slice[0], slice[1], slice[2], slice[3])
    }

    fn as_slice(&self) -> &[f32; 4] {
        unsafe { &*(self.as_ptr() as *const [f32; 4]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const glam::Vec4 as *const f32
    }
}

impl Zero for glam::Vec4 {
    fn zero() -> Self {
        glam::Vec4::ZERO
    }
}

impl Vec4<f64> for glam::DVec4 {
    fn from_slice(slice: &[f64; 4]) -> Self {
        glam::DVec4::new(slice[0], slice[1], slice[2], slice[3])
    }

    fn as_slice(&self) -> &[f64; 4] {
        unsafe { &*(self.as_ptr() as *const [f64; 4]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const glam::DVec4 as *const f64
    }
}

impl Zero for glam::DVec4 {
    fn zero() -> Self {
        glam::DVec4::ZERO
    }
}

impl Vec4<i32> for glam::IVec4 {
    fn from_slice(slice: &[i32; 4]) -> Self {
        glam::IVec4::new(slice[0], slice[1], slice[2], slice[3])
    }

    fn as_slice(&self) -> &[i32; 4] {
        unsafe { &*(self.as_ptr() as *const [i32; 4]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const glam::IVec4 as *const i32
    }
}

impl Zero for glam::IVec4 {
    fn zero() -> Self {
        glam::IVec4::ZERO
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box2f {
    pub min: glam::Vec2,
    pub max: glam::Vec2,
}

impl Bound2<f32> for Box2f {
    fn from_slice(slice: &[f32; 4]) -> Self {
        Box2f {
            min: glam::Vec2::new(slice[0], slice[1]),
            max: glam::Vec2::new(slice[2], slice[3]),
        }
    }

    fn as_slice(&self) -> &[f32; 4] {
        unsafe { &*(self.as_ptr() as *const [f32; 4]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const Box2f as *const f32
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box2d {
    pub min: glam::DVec2,
    pub max: glam::DVec2,
}

impl Bound2<f64> for Box2d {
    fn from_slice(slice: &[f64; 4]) -> Self {
        Box2d {
            min: glam::DVec2::new(slice[0], slice[1]),
            max: glam::DVec2::new(slice[2], slice[3]),
        }
    }

    fn as_slice(&self) -> &[f64; 4] {
        unsafe { &*(self.as_ptr() as *const [f64; 4]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const Box2d as *const f64
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box2i {
    pub min: glam::IVec2,
    pub max: glam::IVec2,
}

impl Bound2<i32> for Box2i {
    fn from_slice(slice: &[i32; 4]) -> Self {
        Box2i {
            min: glam::IVec2::new(slice[0], slice[1]),
            max: glam::IVec2::new(slice[2], slice[3]),
        }
    }

    fn as_slice(&self) -> &[i32; 4] {
        unsafe { &*(self.as_ptr() as *const [i32; 4]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const Box2i as *const i32
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box3f {
    pub min: glam::Vec3,
    pub max: glam::Vec3,
}

impl Bound3<f32> for Box3f {
    fn from_slice(slice: &[f32; 6]) -> Self {
        Box3f {
            min: glam::Vec3::new(slice[0], slice[1], slice[2]),
            max: glam::Vec3::new(slice[3], slice[4], slice[5]),
        }
    }

    fn as_slice(&self) -> &[f32; 6] {
        unsafe { &*(self.as_ptr() as *const [f32; 6]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const Box3f as *const f32
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box3d {
    pub min: glam::DVec3,
    pub max: glam::DVec3,
}

impl Bound3<f64> for Box3d {
    fn from_slice(slice: &[f64; 6]) -> Self {
        Box3d {
            min: glam::DVec3::new(slice[0], slice[1], slice[2]),
            max: glam::DVec3::new(slice[3], slice[4], slice[5]),
        }
    }

    fn as_slice(&self) -> &[f64; 6] {
        unsafe { &*(self.as_ptr() as *const [f64; 6]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const Box3d as *const f64
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box3i {
    pub min: glam::IVec3,
    pub max: glam::IVec3,
}

impl Bound3<i32> for Box3i {
    fn from_slice(slice: &[i32; 6]) -> Self {
        Box3i {
            min: glam::IVec3::new(slice[0], slice[1], slice[2]),
            max: glam::IVec3::new(slice[3], slice[4], slice[5]),
        }
    }

    fn as_slice(&self) -> &[i32; 6] {
        unsafe { &*(self.as_ptr() as *const [i32; 6]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const Box3i as *const i32
    }
}

#[cfg(test)]
#[test]
fn test_box_glam() {
    use glam::{IVec2, IVec3};

    let b2 = Box2i {
        min: IVec2::new(0, 0),
        max: IVec2::new(5, 7),
    };

    assert_eq!(b2.width(), 5);
    assert_eq!(b2.height(), 7);

    let b3 = Box3i {
        min: IVec3::new(0, 0, 0),
        max: IVec3::new(5, 7, 9),
    };

    assert_eq!(b3.width(), 5);
    assert_eq!(b3.height(), 7);
    assert_eq!(b3.depth(), 9);
}
