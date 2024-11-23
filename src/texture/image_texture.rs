use super::Texture;
use image::GenericImageView;

struct ImageData {
    width: u32,
    height: u32,
    data: Vec<u8>,
}
fn load_image(path: &str) -> ImageData {
    let img = image::open(path);
    if img.is_err() {
        panic!("Error: Failed to open image: {}", path);
    }

    let img = img.unwrap();
    let (width, height) = img.dimensions();
    let pixel_data = img.to_rgb8().into_raw();

    ImageData {
        width,
        height,
        data: pixel_data,
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: u32 = 3;
    pub fn new(filename: &str) -> Self {
        let data = load_image(filename);
        ImageTexture {
            data: data.data,
            width: data.width,
            height: data.height,
            bytes_per_scanline: Self::BYTES_PER_PIXEL * data.width,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &crate::vec3::Point3) -> crate::vec3::Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let color_scale = 1.0 / 255.0;
        let index = j * self.bytes_per_scanline + i * Self::BYTES_PER_PIXEL;
        let pixel: &[u8] = &self.data[index as usize..(index + Self::BYTES_PER_PIXEL) as usize];
        crate::vec3::Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
