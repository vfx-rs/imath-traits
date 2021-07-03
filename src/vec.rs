pub trait Vec2<T> {
    fn from_slice(slice: &[T; 2]) -> Self
    where
        Self: Sized;
    fn as_slice(&self) -> &[T; 2];
    fn as_ptr(&self) -> *const T;
}

impl<T> Vec2<T> for [T; 2]
where
    T: Copy,
    Self: Sized,
{
    fn from_slice(slice: &[T; 2]) -> [T; 2] {
        *slice
    }

    fn as_slice(&self) -> &[T; 2] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}

pub trait Vec3<T>
where
    Self: Sized,
{
    fn from_slice(slice: &[T; 3]) -> Self;
    fn as_slice(&self) -> &[T; 3];
    fn as_ptr(&self) -> *const T;
}

impl<T> Vec3<T> for [T; 3]
where
    T: Copy,
{
    fn from_slice(slice: &[T; 3]) -> Self {
        *slice
    }

    fn as_slice(&self) -> &[T; 3] {
        self
    }

    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}

pub trait Vec4<T>
where
    Self: Sized,
{
    fn from_slice(slice: &[T; 4]) -> Self;
    fn as_slice(&self) -> &[T; 4];
    fn as_ptr(&self) -> *const T;
}

impl<T> Vec4<T> for [T; 4]
where
    T: Copy,
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
