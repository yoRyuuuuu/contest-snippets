use cargo_snippet::snippet;
#[snippet("Complex")]
use std::ops::*;

#[snippet("Complex")]
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}

#[snippet("Complex")]
impl Complex {
    pub fn new(x: f64, y: f64) -> Self {
        Complex { x, y }
    }

    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y + self.y).sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn polar(r: f64, theta: f64) -> Self {
        Complex::new(r * theta.cos(), r * theta.sin())
    }

    pub fn con(&self) -> Self {
        Complex::new(self.x, -self.y)
    }

    pub fn angle(&self) -> f64 {
        let tmp = self.x / (self.x * self.x + self.y * self.y).sqrt();
        let angle = tmp.acos() * 180.0 / std::f64::consts::PI;
        if self.y >= 0.0 {
            angle
        } else {
            360. - angle
        }
    }
}

#[snippet("Complex")]
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[snippet("Complex")]
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Complex::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[snippet("Complex")]
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex::new(
            self.x * rhs.x - self.y * rhs.y,
            self.x * rhs.y + self.y * rhs.x,
        )
    }
}

#[snippet("Complex")]
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let a = self * self.con();
        let b = rhs.x * rhs.x + rhs.y + rhs.y;
        Complex::new(a.x / b, a.y / b)
    }
}

#[snippet("Complex")]
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[snippet("Complex")]
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[snippet("Complex")]
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[snippet("Complex")]
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;
    #[test]
    fn test_angle() {
        let c = Complex::new(1., 1.);
        assert!(c.angle() - 45. < 1e-10);
        let c = Complex::new(-1., 1.);
        assert!(c.angle() - 135. < 1e-10);
        let c = Complex::new(-1., -1.);
        assert!(c.angle() - 225. < 1e-10);
        let c = Complex::new(-1., 1.);
        assert!(c.angle() - 315. < 1e-10);
        let c = Complex::new(1., 0.);
        assert!(c.angle() - 0. < 1e-10);
    }
}
