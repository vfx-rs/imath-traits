use half::f16;

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f16 {
    fn zero() -> Self {
        f16::default()
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0f32
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0f64
    }
}

impl Zero for i16 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

impl Zero for u16 {
    fn zero() -> Self {
        0
    }
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}
