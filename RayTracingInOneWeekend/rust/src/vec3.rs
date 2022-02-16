use std::ops;

pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn default() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3(e0, e1, e2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        f64::powi(self.0, 2) + f64::powi(self.1, 2) + f64::powi(self.2, 2)
    }

    pub fn cross(&self, _rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * _rhs.z() - self.z() * _rhs.y(),
            self.z() * _rhs.x() - self.x() * _rhs.z(),
            self.x() * _rhs.y() - self.y() * _rhs.x(),
        )
    }

    pub fn dot(&self, _rhs: Vec3) -> f64 {
        self.x() * _rhs.x() + self.y() * _rhs.y() + self.z() * _rhs.z()
    }
}

// Overloading methods

// + operation
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x() + _rhs.x(),
            self.y() + _rhs.y(),
            self.z() + _rhs.z(),
        )
    }
}

// - (subtract) operation
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x() - _rhs.x(),
            self.y() - _rhs.y(),
            self.z() - _rhs.z(),
        )
    }
}

// - (negate) operation
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// * (multiply) operation
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x() * _rhs, self.y() * _rhs, self.z() * _rhs)
    }
}

// * (multiply) operation
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Vec3 {
        self * (1.0 / _rhs)
    }
}

// += operation
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2)
    }
}

// *= operation
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        *self = Self(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        *self *= 1.0 / _rhs
    }
}

// Type aliases
pub use Vec3 as Point3;
pub use Vec3 as Color;

// Color utility methods
impl Color {
    pub fn r(&self) -> f64 {
        self.0
    }
    pub fn g(&self) -> f64 {
        self.1
    }
    pub fn b(&self) -> f64 {
        self.2
    }
}
