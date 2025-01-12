use image::{io::Reader as ImageReader, GenericImageView};
use std::path::PathBuf;

pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Image {
    pub image: image::DynamicImage,
    pub path: PathBuf,
    pub dimensions: ImageDimensions,
    pub image_name: String,
    pub image_extension: String,
}

impl Image {
    pub fn read_image(path: PathBuf) -> Self {
        let image = ImageReader::open(&path)
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to decode image");
        let (width, height) = image.dimensions();
        let dimensions = ImageDimensions { height, width };

        let binding = path.clone();
        let image_name = binding.to_str().unwrap().split('/').last().unwrap();
        let image_extension = path.extension().unwrap().to_str().unwrap().to_string();
        let image_name = image_name.split('.').next().unwrap().to_string();
        Self {
            image,
            path,
            dimensions,
            image_name,
            image_extension,
        }
    }

    pub fn save_image(image: &image::DynamicImage, path: PathBuf) {
        image
            .save_with_format(path, image::ImageFormat::Png)
            .expect("Failed to save image");
    }
}
