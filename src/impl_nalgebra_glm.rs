//! Implements all traits for nalgebra-glm
//!
//! Impls `Vec2`, `Vec3`, `Vec3A`, `Vec4`
//! Defines `Box2<T>` and `Box3<T>` in terms of `Vec2` and `Vec3`, respectively.

pub use crate::*;

use nalgebra_glm as glm;

impl Vec2<f32> for glm::Vec2 {
    fn from_slice(slice: &[f32; 2]) -> Self {
        glm::Vec2::new(slice[0], slice[1])
    }

    fn as_slice(&self) -> &[f32; 2] {
        unsafe { &*(self.as_ptr() as *const [f32; 2]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const glm::Vec2 as *const f32
    }
}

impl Zero for glm::Vec2 {
    fn zero() -> Self {
        glm::Vec2::new(0.0, 0.0)
    }
}

impl Vec2<f64> for glm::DVec2 {
    fn from_slice(slice: &[f64; 2]) -> Self {
        glm::DVec2::new(slice[0], slice[1])
    }

    fn as_slice(&self) -> &[f64; 2] {
        unsafe { &*(self.as_ptr() as *const [f64; 2]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const glm::DVec2 as *const f64
    }
}

impl Zero for glm::DVec2 {
    fn zero() -> Self {
        glm::DVec2::new(0.0, 0.0)
    }
}

impl Vec2<i32> for glm::IVec2 {
    fn from_slice(slice: &[i32; 2]) -> Self {
        glm::IVec2::new(slice[0], slice[1])
    }

    fn as_slice(&self) -> &[i32; 2] {
        unsafe { &*(self.as_ptr() as *const [i32; 2]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const glm::IVec2 as *const i32
    }
}

impl Zero for glm::IVec2 {
    fn zero() -> Self {
        glm::IVec2::new(0, 0)
    }
}

impl Vec3<f32> for glm::Vec3 {
    fn from_slice(slice: &[f32; 3]) -> Self {
        glm::Vec3::new(slice[0], slice[1], slice[2])
    }

    fn as_slice(&self) -> &[f32; 3] {
        unsafe { &*(self.as_ptr() as *const [f32; 3]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const glm::Vec3 as *const f32
    }
}

impl Zero for glm::Vec3 {
    fn zero() -> Self {
        glm::Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Vec3<f64> for glm::DVec3 {
    fn from_slice(slice: &[f64; 3]) -> Self {
        glm::DVec3::new(slice[0], slice[1], slice[2])
    }

    fn as_slice(&self) -> &[f64; 3] {
        unsafe { &*(self.as_ptr() as *const [f64; 3]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const glm::DVec3 as *const f64
    }
}

impl Zero for glm::DVec3 {
    fn zero() -> Self {
        glm::DVec3::new(0.0, 0.0, 0.0)
    }
}

impl Vec3<i32> for glm::IVec3 {
    fn from_slice(slice: &[i32; 3]) -> Self {
        glm::IVec3::new(slice[0], slice[1], slice[2])
    }

    fn as_slice(&self) -> &[i32; 3] {
        unsafe { &*(self.as_ptr() as *const [i32; 3]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const glm::IVec3 as *const i32
    }
}

impl Zero for glm::IVec3 {
    fn zero() -> Self {
        glm::IVec3::new(0, 0, 0)
    }
}

impl Vec4<f32> for glm::Vec4 {
    fn from_slice(slice: &[f32; 4]) -> Self {
        glm::Vec4::new(slice[0], slice[1], slice[2], slice[3])
    }

    fn as_slice(&self) -> &[f32; 4] {
        unsafe { &*(self.as_ptr() as *const [f32; 4]) }
    }

    fn as_ptr(&self) -> *const f32 {
        self as *const glm::Vec4 as *const f32
    }
}

impl Zero for glm::Vec4 {
    fn zero() -> Self {
        glm::Vec4::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Vec4<f64> for glm::DVec4 {
    fn from_slice(slice: &[f64; 4]) -> Self {
        glm::DVec4::new(slice[0], slice[1], slice[2], slice[3])
    }

    fn as_slice(&self) -> &[f64; 4] {
        unsafe { &*(self.as_ptr() as *const [f64; 4]) }
    }

    fn as_ptr(&self) -> *const f64 {
        self as *const glm::DVec4 as *const f64
    }
}

impl Zero for glm::DVec4 {
    fn zero() -> Self {
        glm::DVec4::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Vec4<i32> for glm::IVec4 {
    fn from_slice(slice: &[i32; 4]) -> Self {
        glm::IVec4::new(slice[0], slice[1], slice[2], slice[3])
    }

    fn as_slice(&self) -> &[i32; 4] {
        unsafe { &*(self.as_ptr() as *const [i32; 4]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const glm::IVec4 as *const i32
    }
}

impl Zero for glm::IVec4 {
    fn zero() -> Self {
        glm::IVec4::new(0, 0, 0, 0)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box2f {
    pub min: glm::Vec2,
    pub max: glm::Vec2,
}

impl Bound2<f32> for Box2f {
    fn from_slice(slice: &[f32; 4]) -> Self {
        Box2f {
            min: glm::Vec2::new(slice[0], slice[1]),
            max: glm::Vec2::new(slice[2], slice[3]),
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
    pub min: glm::DVec2,
    pub max: glm::DVec2,
}

impl Bound2<f64> for Box2d {
    fn from_slice(slice: &[f64; 4]) -> Self {
        Box2d {
            min: glm::DVec2::new(slice[0], slice[1]),
            max: glm::DVec2::new(slice[2], slice[3]),
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
    pub min: glm::IVec2,
    pub max: glm::IVec2,
}

impl Bound2<i32> for Box2i {
    fn from_slice(slice: &[i32; 4]) -> Self {
        Box2i {
            min: glm::IVec2::new(slice[0], slice[1]),
            max: glm::IVec2::new(slice[2], slice[3]),
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
    pub min: glm::Vec3,
    pub max: glm::Vec3,
}

impl Bound3<f32> for Box3f {
    fn from_slice(slice: &[f32; 6]) -> Self {
        Box3f {
            min: glm::Vec3::new(slice[0], slice[1], slice[2]),
            max: glm::Vec3::new(slice[3], slice[4], slice[5]),
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
    pub min: glm::DVec3,
    pub max: glm::DVec3,
}

impl Bound3<f64> for Box3d {
    fn from_slice(slice: &[f64; 6]) -> Self {
        Box3d {
            min: glm::DVec3::new(slice[0], slice[1], slice[2]),
            max: glm::DVec3::new(slice[3], slice[4], slice[5]),
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
    pub min: glm::IVec3,
    pub max: glm::IVec3,
}

impl Bound3<i32> for Box3i {
    fn from_slice(slice: &[i32; 6]) -> Self {
        Box3i {
            min: glm::IVec3::new(slice[0], slice[1], slice[2]),
            max: glm::IVec3::new(slice[3], slice[4], slice[5]),
        }
    }

    fn as_slice(&self) -> &[i32; 6] {
        unsafe { &*(self.as_ptr() as *const [i32; 6]) }
    }

    fn as_ptr(&self) -> *const i32 {
        self as *const Box3i as *const i32
    }
}
