use crate::hittable::Hittable;
use crate::vec3::Point3;

#[derive(Default, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: Double,
}

impl Sphere for Hittable {
    pub fn default() -> Sphere {
        Sphere {
            center: Point3::default(),
            radius: 0.0,
        }
    }

    pub fn new(cen: Point3, r: Double) -> Sphere {
        Self {
            center: cen,
            radius: r,
        }
    }

    pub fn hit(&self, r: Ray, t_min: Double, t_max: Double, rec: Hit_Record) -> bool {
        oc: Vec3 = r.origin() - self.center;

        false
    }
}
