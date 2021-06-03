pub trait Number {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Number for i64 {
    fn zero() -> Self {
        0 as Self
    }

    fn one() -> Self {
        1 as Self
    }
}

impl Number for f64 {
    fn zero() -> Self {
        0 as Self
    }

    fn one() -> Self {
        1 as Self
    }
}
