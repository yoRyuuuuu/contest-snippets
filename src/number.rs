pub trait Number {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Number for i64 {
    fn zero() -> Self {
        0_i64
    }

    fn one() -> Self {
        1_i64
    }
}

impl Number for f64 {
    fn zero() -> Self {
        0_f64
    }

    fn one() -> Self {
        1_f64
    }
}
