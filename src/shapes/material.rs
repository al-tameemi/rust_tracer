use crate::{
    primitives::{color::Color, ray::Ray, vector::Vector}, 
    shapes::hittable::HitRecord
};

#[derive(Clone, Copy, Debug)]
pub enum MaterialType{
    Lambertian,
    Metal
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub albedo: Color,
    pub mat_type: MaterialType
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Material {
        Material {
            albedo,
            mat_type: MaterialType::Lambertian
        }
    }

    pub fn new_metal(albedo: Color) -> Material {
        Material {
            albedo,
            mat_type: MaterialType::Metal
        }
    }

    pub fn new(albedo: Color, mat_type: MaterialType) -> Material {
        Material {
            albedo,
            mat_type
        }
    }

    pub fn scatter(
        &self, ray_in: &Ray, record: &HitRecord, 
        attenuation: &mut Color, scattered: &mut Ray
    ) -> bool {
        match self.mat_type {
            MaterialType::Lambertian => {
                return self.scatter_lambertian(ray_in, record, attenuation, scattered);
            },
            MaterialType::Metal => {
                return self.scatter_metal(ray_in, record, attenuation, scattered);
            }
        }
    }

    pub fn scatter_lambertian(
        &self, ray_in: &Ray, record: &HitRecord, 
        attenuation: &mut Color, scattered: &mut Ray
    ) -> bool{
        let mut scatter_direction = record.normal.unwrap() + Vector::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal.unwrap();
        }

        *scattered = Ray::new(record.point.unwrap(), scatter_direction);
        *attenuation = self.albedo;
        return true;
    }

    pub fn scatter_metal(
        &self, ray_in: &Ray, record: &HitRecord, 
        attenuation: &mut Color, scattered: &mut Ray
    ) -> bool{
        let reflected = ray_in.direction.unit_vector().reflect(&record.normal.unwrap());
        *scattered = Ray::new(record.point.unwrap(), reflected);
        *attenuation = self.albedo;
        
        scattered.direction.dot(&record.normal.unwrap()) > 0.0
    }
}