use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn default() -> Ray {
        Ray(Point3.default(), Vec3.default())
    }

    pub fn new(e0: Point3, e1: Vec3) -> Self {
        Self {
            orig: e0,
            dir: e1
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }

}