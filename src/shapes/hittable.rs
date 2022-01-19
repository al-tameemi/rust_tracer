use crate::primitives::{ray::Ray, vector::Vector};
use std::rc::Rc;
use super::material::Material;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: Option<Vector>,
    pub normal: Option<Vector>,
    pub t: Option<f64>,
    pub front_face: Option<bool>,
    pub material: Option<Material>
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: None,
            normal: None,
            t: None,
            front_face: None,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector) {
        self.front_face = Some(ray.direction.dot(&outward_normal) < 0.0);
        self.normal = match self.front_face {
            Some(boolean) => {
                match boolean {
                    true => {Some(outward_normal)},
                    false => {Some(-outward_normal)}
                }
            },
            _ => {None}
        };
    }
}