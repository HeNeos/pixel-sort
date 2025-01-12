mod models;
mod processing;

use clap::Parser;

use crate::models::cli::Cli;
use crate::models::image::Image;
use crate::processing::image::pixel_sorting;

fn main() {
    let args = Cli::parse();
    let image: Image = Image::read_image(args.image_path);
    pixel_sorting(image);
}
