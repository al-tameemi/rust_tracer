mod primitives;
mod objects;

use primitives::{color::Color, vector::{Vector, Vec3}, ray::Ray};
use objects::{image::Image, camera::Camera};
use image::{RgbImage, Rgb};

fn main() {

    let image = Image::new_with_height(16.0 / 9.0, 1440);
    let camera = Camera::from_image(&image);

    let mut rgb_image = RgbImage::new(image.width as u32, image.height as u32);

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let u = i as f64 / (image.width - 1) as f64;
            let v = j as f64 / (image.height - 1) as f64;
            let ray = Ray::new(camera.origin, camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin);
            let color = ray_color(ray);
            rgb_image.put_pixel(i as u32, (image.height - 1 - j) as u32  , Rgb(color.pixels()));
        }
    }

    rgb_image.save("image.png").unwrap();
}

fn ray_color(ray: Ray) -> Color {
    let t = hits_sphere(ray, Vector::new(0.0, 0.0, -1.0), 0.5);
    if t > 0.0 {
        let n = (ray.at(t) - Vector::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn hits_sphere(ray: Ray, center: Vector, radius: f64) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (a);
    }
}