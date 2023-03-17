use std::ops;

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            x: e0,
            y: e1,
            z: e2,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        f64::powi(self.x, 2) + f64::powi(self.y, 2) + f64::powi(self.z, 2)
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
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x() * _rhs,
            y: self.y() * _rhs,
            z: self.z() * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x(),
            y: self * _rhs.y(),
            z: self * _rhs.z(),
        }
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
        *self = Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

// *= operation
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        *self = Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
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
        self.x
    }
    pub fn g(&self) -> f64 {
        self.y
    }
    pub fn b(&self) -> f64 {
        self.z
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
