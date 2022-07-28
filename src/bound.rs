use std::fmt;
use std::ops::Sub;

pub trait Bound1<T>
where
    Self: Sized,
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 2]) -> Self;
    fn as_slice(&self) -> &[T; 2];
    fn as_ptr(&self) -> *const T;

    fn width(&self) -> T {
        let s = self.as_slice();
        s[1] - s[0]
    }

    fn bound_min(&self) -> T {
        let s = self.as_slice();
        s[0]
    }

    fn bound_max(&self) -> T {
        let s = self.as_slice();
        s[1]
    }
}

impl<T> Bound1<T> for [T; 2]
where
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 2]) -> Self {
        *slice
    }

    fn as_slice(&self) -> &[T; 2] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}

pub trait Bound2<T>
where
    Self: Sized,
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 4]) -> Self;
    fn as_slice(&self) -> &[T; 4];
    fn as_ptr(&self) -> *const T;

    fn width(&self) -> T {
        let s = self.as_slice();
        s[2] - s[0]
    }

    fn height(&self) -> T {
        let s = self.as_slice();
        s[3] - s[1]
    }

    fn min_x(&self) -> T {
        let s = self.as_slice();
        s[0]
    }

    fn min_y(&self) -> T {
        let s = self.as_slice();
        s[1]
    }

    fn max_x(&self) -> T {
        let s = self.as_slice();
        s[2]
    }

    fn max_y(&self) -> T {
        let s = self.as_slice();
        s[3]
    }
}

impl<T> Bound2<T> for [T; 4]
where
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 4]) -> Self {
        *slice
    }

    fn as_slice(&self) -> &[T; 4] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}

pub trait Bound3<T>
where
    Self: Sized,
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 6]) -> Self;
    fn as_slice(&self) -> &[T; 6];
    fn as_ptr(&self) -> *const T;

    fn width(&self) -> T {
        let s = self.as_slice();
        s[3] - s[0]
    }

    fn height(&self) -> T {
        let s = self.as_slice();
        s[4] - s[1]
    }

    fn depth(&self) -> T {
        let s = self.as_slice();
        s[5] - s[2]
    }
}

impl<T> Bound3<T> for [T; 6]
where
    T: Copy,
    T: Sub<Output = T> + Copy + fmt::Debug,
{
    fn from_slice(slice: &[T; 6]) -> Self {
        *slice
    }

    fn as_slice(&self) -> &[T; 6] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}
