use num::traits::Pow;

use crate::{
    primitives::{color::Color, ray::Ray, vector::Vector}, 
    shapes::hitrecord::HitRecord
};

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum MaterialType{
    Lambertian,
    Metal,
    Dielectric
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub albedo: Color,
    pub fuzz: f64,
    pub mat_type: MaterialType,
    pub index_of_refraction: f64,
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Material {
        Material {
            albedo,
            fuzz: 0.0,
            mat_type: MaterialType::Lambertian,
            index_of_refraction: 0.0
        }
    }

    pub fn new_metal(albedo: Color, mut fuzz: f64) -> Material {
        if fuzz > 1.0 {
            fuzz = 1.0;
        }
        Material {
            albedo,
            mat_type: MaterialType::Metal,
            fuzz,
            index_of_refraction: 0.0
        }
    }

    pub fn new_dielectric(albedo: Color, index_of_refraction: f64) -> Material {
        Material {
            albedo,
            mat_type: MaterialType::Dielectric,
            fuzz: 0.0,
            index_of_refraction
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
            },
            MaterialType::Dielectric => {
                return self.scatter_dielectric(ray_in, record, attenuation, scattered)
            }
        }
    }

    pub fn scatter_lambertian(
        &self, _ray_in: &Ray, record: &HitRecord, 
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
        *scattered = Ray::new(record.point.unwrap(), reflected + self.fuzz * Vector::random_unit_vector());
        *attenuation = self.albedo;
        
        scattered.direction.dot(&record.normal.unwrap()) > 0.0
    }

    pub fn scatter_dielectric(
        &self, ray_in: &Ray, record: &HitRecord, 
        attenuation: &mut Color, scattered: &mut Ray
    ) -> bool{
        let reflectance = |cosine: f64, ref_idx: f64| -> f64{
            let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            r0 = r0 * r0;
            r0 + (1.0 - r0) * (1.0 - cosine).pow(5)
        };

        *attenuation = self.albedo;
        let refraction_ratio = match record.front_face.unwrap() {
            true => {1.0 / self.index_of_refraction},
            false => {self.index_of_refraction}
        };

        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = -unit_direction.dot(&record.normal.unwrap()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = refraction_ratio * sin_theta > 1.0;

        let direction;

        if cannot_reflect || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen() {
            direction = unit_direction.reflect(&record.normal.unwrap());
        } else {
            direction = unit_direction.refract(&record.normal.unwrap(), refraction_ratio);
        }

        *scattered = Ray::new(record.point.unwrap(), direction);
        true
    }
}