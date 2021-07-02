//! Implements all traits for nalgebra
//!
//! Impls `Vector2<T>`, `Vector3<T>`, `Vector4<T>`.
//! Defines `Box2<T>` and `Box3<T>` in terms of `Vector2<T>` and `Vector3<T>`, respectively.

pub use crate::*;

use nalgebra::{vector, Vector2, Vector3, Vector4};

impl<T> Vec2<T> for nalgebra::Vector2<T>
where
    T: Copy,
{
    fn from_slice(slice: &[T; 2]) -> Vector2<T> {
        vector![slice[0], slice[1]]
    }

    fn as_slice(&self) -> &[T; 2] {
        unsafe { &*(self.as_ptr() as *const [T; 2]) }
    }

    fn as_ptr(&self) -> *const T {
        self as *const Vector2<T> as *const T
    }
}

impl<T> Zero for Vector2<T>
where
    T: Copy + Zero,
{
    fn zero() -> Self {
        vector![T::zero(), T::zero()]
    }
}

impl<T> Vec3<T> for nalgebra::Vector3<T>
where
    T: Copy,
{
    fn from_slice(slice: &[T; 3]) -> Vector3<T> {
        vector![slice[0], slice[1], slice[2]]
    }

    fn as_slice(&self) -> &[T; 3] {
        unsafe { &*(self.as_ptr() as *const [T; 3]) }
    }

    fn as_ptr(&self) -> *const T {
        self as *const Vector3<T> as *const T
    }
}

impl<T> Zero for Vector3<T>
where
    T: Copy + Zero,
{
    fn zero() -> Self {
        vector![T::zero(), T::zero(), T::zero()]
    }
}

impl<T> Vec4<T> for nalgebra::Vector4<T>
where
    T: Copy,
{
    fn from_slice(slice: &[T; 4]) -> Vector4<T> {
        vector![slice[0], slice[1], slice[2], slice[3]]
    }

    fn as_slice(&self) -> &[T; 4] {
        unsafe { &*(self.as_ptr() as *const [T; 4]) }
    }

    fn as_ptr(&self) -> *const T {
        self as *const Vector4<T> as *const T
    }
}

impl<T> Zero for Vector4<T>
where
    T: Copy + Zero,
{
    fn zero() -> Self {
        vector![T::zero(), T::zero(), T::zero(), T::zero()]
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box2<T>
where
    T: Copy + std::fmt::Debug + PartialEq + 'static,
{
    pub min: Vector2<T>,
    pub max: Vector2<T>,
}

impl<T> Bound2<T> for Box2<T>
where
    T: Copy + std::fmt::Debug + PartialEq + 'static,
{
    fn from_slice(slice: &[T; 4]) -> Self {
        Self {
            min: vector![slice[0], slice[1]],
            max: vector![slice[2], slice[3]],
        }
    }

    fn as_slice(&self) -> &[T; 4] {
        unsafe { &*(self.as_ptr() as *const [T; 4]) }
    }

    fn as_ptr(&self) -> *const T {
        self as *const Box2<T> as *const T
    }
}

pub type Box2i = Box2<i32>;
pub type Box2f = Box2<f32>;
pub type Box2d = Box2<f64>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Box3<T>
where
    T: Copy + std::fmt::Debug + PartialEq + 'static,
{
    pub min: Vector3<T>,
    pub max: Vector3<T>,
}

impl<T> Bound3<T> for Box3<T>
where
    T: Copy + std::fmt::Debug + PartialEq + 'static,
{
    fn from_slice(slice: &[T; 6]) -> Self {
        Self {
            min: vector![slice[0], slice[1], slice[2]],
            max: vector![slice[3], slice[4], slice[5]],
        }
    }

    fn as_slice(&self) -> &[T; 6] {
        unsafe { &*(self.as_ptr() as *const [T; 6]) }
    }

    fn as_ptr(&self) -> *const T {
        self as *const Box3<T> as *const T
    }
}

pub type Box3i = Box3<i32>;
pub type Box3f = Box3<f32>;
pub type Box3d = Box3<f64>;
