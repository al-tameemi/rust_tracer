mod vec3;

use vec3::{color::Color, point::Point};

const IMAGE_WIDTH: i32 = 1024;
const IMAGE_HEIGHT: i32 = 1024;

fn main() {
    let mut ppm_image = format!("P3\n{} {}\n255\n", IMAGE_WIDTH,  IMAGE_HEIGHT);
    
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            // println!("at row: {i} col: {j}");
            let color = Color::new(i as f64 / (IMAGE_WIDTH - 1) as f64, j as f64 / (IMAGE_HEIGHT - 1) as f64, 0.25);
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