mod primitives;
mod objects;

use primitives::{color::Color, vec::{Vec, Vec3}, ray::Ray};
use objects::{image::Image, camera::Camera};

fn main() {

    let image = Image::new();
    let camera = Camera::from_image(&image);

    let mut ppm_image = format!("P3\n{} {}\n255\n", image.width,  image.height);
    
    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let u = i as f64 / (image.width - 1) as f64;
            let v = j as f64 / (image.height - 1) as f64;
            let ray = Ray::new(camera.origin, camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin);
            let color = ray_color(ray);
            ppm_image.push_str(&color.write_color());
        }
    }

    write_ppm("image.ppm", ppm_image).unwrap();
}

use std::{fs::File, io::Write};
fn write_ppm(file: &str, image: String) -> std::io::Result<()>{
    let mut file = File::create(file)?;
    file.write_all(image.as_bytes())?;
    Ok(())
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}