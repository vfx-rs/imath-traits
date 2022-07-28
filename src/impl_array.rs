pub use crate::*;
use std::fmt;
use std::ops::Sub;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Box1<T>
where
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    pub min: T,
    pub max: T,
}

impl<T> Bound1<T> for Box1<T>
where
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 2]) -> Self {
        Box1::<T> {
            min: slice[0],
            max: slice[1],
        }
    }

    fn as_slice(&self) -> &[T; 2] {
        unsafe { &*(self.as_ptr() as *const [T; 2]) }
    }

    fn as_ptr(&self) -> *const T {
        self as *const Box1<T> as *const T
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Box2<T>
where
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    pub min: [T; 2],
    pub max: [T; 2],
}

impl<T> Bound2<T> for Box2<T>
where
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 4]) -> Self {
        Box2::<T> {
            min: [slice[0], slice[1]],
            max: [slice[2], slice[3]],
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

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Box3<T>
where
    T: Copy,
{
    pub min: [T; 3],
    pub max: [T; 3],
}

impl<T> Bound3<T> for Box3<T>
where
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 6]) -> Self {
        Box3::<T> {
            min: [slice[0], slice[1], slice[2]],
            max: [slice[3], slice[4], slice[5]],
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
