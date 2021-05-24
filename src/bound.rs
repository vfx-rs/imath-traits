pub trait Bound2<T>
where
    Self: Sized,
{
    fn from_slice(slice: &[T; 4]) -> Self;
    fn as_slice(&self) -> &[T; 4];
    fn as_ptr(&self) -> *const T;
}

impl<T> Bound2<T> for [T; 4]
where
    T: Copy,
{
    fn from_slice(slice: &[T; 4]) -> Self {
        slice.clone()
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
{
    fn from_slice(slice: &[T; 6]) -> Self;
    fn as_slice(&self) -> &[T; 6];
    fn as_ptr(&self) -> *const T;
}

impl<T> Bound3<T> for [T; 6]
where
    T: Copy,
{
    fn from_slice(slice: &[T; 6]) -> Self {
        slice.clone()
    }

    fn as_slice(&self) -> &[T; 6] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}
