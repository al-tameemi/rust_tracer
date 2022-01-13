mod vec3;

const IMAGE_WIDTH: i32 = 1024;
const IMAGE_HEIGHT: i32 = 1024;

fn main() {
    let mut ppm_image = format!("P3\n{} {}\n255\n", IMAGE_WIDTH,  IMAGE_HEIGHT);
    
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let ir = (255.999 * r) as i32; 
            let ig = (255.999 * g) as i32; 
            let ib = (255.999 * b) as i32;
            
            ppm_image.push_str(& format!("{} {} {}\n", ir, ig, ib));
        }
    }

    print!("{}", ppm_image);
}
