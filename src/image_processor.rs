use image::{DynamicImage, GenericImageView};

use crate::error::ImageProcessingError;

pub type PixelsData = Vec<Vec<i32>>;

#[derive(Debug)]
pub struct ImageProcessor {
    file_path: String,
}

impl ImageProcessor {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    fn get_image(&self) -> Result<DynamicImage, ImageProcessingError> {
        if self.file_path.is_empty() {
            return Err(ImageProcessingError::NullPath);
        }

        // println!("{}", self.file_path);
        let image = image::open(&self.file_path)?;
        Ok(image.resize(80, 80, image::imageops::FilterType::Nearest))
    }

    pub fn get_brightness_matrix(&self) -> PixelsData {
        let image = self.get_image().unwrap();
        let pixels = self.get_pixels_from_image(&image, 0.5).unwrap();

        // println!("pixels: {:?}", pixels);
        self.set_brightness_matrix(pixels)
    }

    fn get_pixels_from_image(
        &self,
        image: &DynamicImage,
        green_filter: f32,
    ) -> Result<PixelsData, ImageProcessingError> {
        let (width, height) = image.dimensions();

        let mut pixels = vec![vec![0; width as usize]; height as usize];

        println!("height: {}, width: {}", height, width);
        for j in 0..height as usize {
            for i in 0..width as usize {
                if i >= width as usize || j >= height as usize {
                    return Err(ImageProcessingError::OutOfBounds);
                }

                let pixel = image.get_pixel(i as u32, j as u32);
                let [r, g, b, _] = pixel.0;

                let filtered_g = (g as f32 * green_filter) as u8;
                let rgb_value = ((r as i32) << 16) | ((filtered_g as i32) << 8) | (b as i32);
                pixels[j][i] = rgb_value;
            }
        }

        Ok(pixels)
    }

    fn set_brightness_matrix(&self, pixel: PixelsData) -> PixelsData {
        let height = pixel.len();
        let width = pixel[0].len();
        let mut pixels = vec![vec![0; width]; height];

        for i in 0..height {
            for j in 0..width {
                let r = (pixel[i][j] >> 16) & 0xff;
                let g = (pixel[i][j] >> 8) & 0xff;
                let b = pixel[i][j] & 0xff;

                let average = (r + g + b) / 3;
                pixels[i][j] = average;
            }
        }

        pixels
    }
}
