use std::ops;


pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn default() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    } 

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3(e0, e1, e2)
    } 

    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
    pub fn z(&self) -> f64 { self.2 }
}

// Overloading methods
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

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3::new(
            -self.x(),
            -self.y(),
            -self.z()
        )
    }
}


// Type aliases
pub use Vec3 as Point3;
pub use Vec3 as Color;

// Color utility methods
impl Color {
    pub fn r(&self) -> f64 { self.0 }
    pub fn g(&self) -> f64 { self.1 }
    pub fn b(&self) -> f64 { self.2 }
}