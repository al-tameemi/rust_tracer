pub struct Image {
    pub aspect_ratio: f64,
    pub width: i32,
    pub height: i32
}

impl Image {
    pub fn new_with_size(width:  i32, height: i32) -> Image {
        let aspect_ratio = width as f64 / height as f64;
        Image {
            aspect_ratio,
            width,
            height
        }
    }

    pub fn new_with_width(aspect_ratio: f64, width: i32) -> Image {
        Image {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as i32
        }
    }

    pub fn new_with_height(aspect_ratio: f64, height: i32) -> Image {
        Image {
            aspect_ratio,
            height,
            width: (height as f64 * aspect_ratio) as i32 
        }
    }
}