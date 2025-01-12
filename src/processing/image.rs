use image::{DynamicImage, GenericImage, GenericImageView};

use crate::models::image::Image;

fn apply_threshold(luminosity: f64) -> Option<f64> {
    if luminosity >= 0.8 || luminosity <= 0.25 {
        None
    } else {
        Some(luminosity)
    }
}

fn calculate_luminosity(pixel: image::Rgba<u8>) -> f64 {
    let r: f64 = pixel[0] as f64 / 255.0;
    let g: f64 = pixel[1] as f64 / 255.0;
    let b: f64 = pixel[2] as f64 / 255.0;

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn sort_by_luminosity(a: &image::Rgba<u8>, b: &image::Rgba<u8>) -> std::cmp::Ordering {
    let lum_a: Option<f64> = apply_threshold(calculate_luminosity(*a));
    let lum_b: Option<f64> = apply_threshold(calculate_luminosity(*b));
    lum_a
        .partial_cmp(&lum_b)
        .unwrap_or(std::cmp::Ordering::Equal)
}

fn sort_interval(chunk: &mut [image::Rgba<u8>], index: usize) -> usize {
    let mut sortable: Vec<image::Rgba<u8>> = Vec::new();
    let start_index: usize = index;
    let mut end_index: usize = index;
    while end_index < chunk.len()
        && apply_threshold(calculate_luminosity(chunk[end_index])).is_some()
    {
        sortable.push(chunk[index]);
        end_index += 1;
    }

    sortable.sort_by(|a, b| sort_by_luminosity(a, b));
    for i in start_index..end_index {
        chunk[i] = sortable[i - start_index];
    }
    end_index
}

pub fn pixel_sorting(image: Image) {
    let (width, height) = (image.dimensions.width, image.dimensions.height);
    let mut new_image = DynamicImage::new_rgb8(width, height);

    for x in 0..width {
        let mut column_pixels = Vec::new();

        for y in 0..height {
            let pixel = new_image.get_pixel(x, y);
            column_pixels.push(pixel);
        }

        let chunk: &mut Vec<image::Rgba<u8>> = &mut column_pixels;
        let mut index: usize = 0;
        while index < chunk.len() {
            let next_index: usize = sort_interval(chunk, index);
            // Threshold pixel
            if next_index == index {
                index += 1;
            } else {
                index = next_index;
            }
        }

        for (y, pixel) in column_pixels.into_iter().enumerate() {
            new_image.put_pixel(x, y as u32, pixel);
        }
    }

    for y in 0..height {
        let mut row_pixels = Vec::new();

        for x in 0..width {
            let pixel = image.image.get_pixel(x, y);
            row_pixels.push(pixel);
        }

        let chunk: &mut Vec<image::Rgba<u8>> = &mut row_pixels;
        let mut index: usize = 0;
        while index < chunk.len() {
            let next_index: usize = sort_interval(chunk, index);
            // Threshold pixel
            if next_index == index {
                index += 1;
            } else {
                index = next_index;
            }
        }

        for (x, pixel) in chunk.into_iter().enumerate() {
            new_image.put_pixel(x as u32, y, *pixel);
        }
    }

    let mut save_path = image.path.parent().unwrap_or(&image.path).to_path_buf();
    save_path.push(format!("{}_sorted", image.image_name));
    save_path.set_extension(image.image_extension);
    println!("Saving image at: {:?}", save_path);

    Image::save_image(&new_image, save_path);
}
