use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Hit_Record {
    p: Point3,
    normal: Vec3,
    t: Double,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: Double, t_max: Double, rec: Hit_Record) -> bool;
}
