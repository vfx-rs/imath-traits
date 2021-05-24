pub trait Matrix33<T>
where
    Self: Sized,
{
    fn from_slice(slice: &[T; 9]) -> Self;
    fn as_slice(&self) -> &[T; 9];
    fn as_ptr(&self) -> *const T;
}

impl<T> Matrix33<T> for [T; 9]
where
    T: Copy,
{
    fn from_slice(slice: &[T; 9]) -> Self {
        slice.clone()
    }

    fn as_slice(&self) -> &[T; 9] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}

pub trait Matrix44<T>
where
    Self: Sized,
{
    fn from_slice(slice: &[T; 16]) -> Self;
    fn as_slice(&self) -> &[T; 16];
    fn as_ptr(&self) -> *const T;
}

impl<T> Matrix44<T> for [T; 16]
where
    T: Copy,
{
    fn from_slice(slice: &[T; 16]) -> Self {
        slice.clone()
    }

    fn as_slice(&self) -> &[T; 16] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}
