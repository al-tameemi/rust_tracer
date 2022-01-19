use rayon::str::MatchIndices;

use super::{hittable::{Hittable, HitRecord}, material::Material};
use crate::primitives::{vector::Vector, ray::Ray};
pub struct Sphere {
    radius: f64,
    center: Vector,
    material: Material
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Material) -> Sphere {
        Sphere { 
            radius, 
            center,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, mut hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();


        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = Some(root);
        hit_record.point = Some(ray.at(root));
        let outward_normal = (hit_record.point.unwrap() - self.center) / self.radius;
        hit_record.set_face_normal(&ray, outward_normal);
        hit_record.material = Some(self.material);
        return true;
    }
}