use anyhow::{anyhow, Result};
use colored::Colorize;

use std::fs::File;

use image_crate::codecs::{bmp::BmpDecoder, jpeg::JpegDecoder, png::PngDecoder};
use printpdf::{image_crate, Image};

enum ImageType {
    BMP,
    JPEG,
    PNG,
    UNSUPPORTED,
}

fn get_image_type(img_file_name: &str) -> ImageType {
    let extension_option = img_file_name.split('.').last();
    if let Some(extension) = extension_option {
        return match extension.to_lowercase().as_str() {
            "bmp" => ImageType::BMP,
            "png" => ImageType::PNG,
            "jpg" => ImageType::JPEG,
            "jpeg" => ImageType::JPEG,
            _ => ImageType::UNSUPPORTED,
        };
    }
    ImageType::UNSUPPORTED
}

pub fn read_image_from_file(img_file_name: &str) -> Result<Image> {
    let mut img_file = File::open(&img_file_name)?;

    match get_image_type(img_file_name) {
        ImageType::BMP => Ok(Image::try_from(BmpDecoder::new(&mut img_file)?)?),
        ImageType::PNG => Ok(Image::try_from(PngDecoder::new(&mut img_file)?)?),
        ImageType::JPEG => Ok(Image::try_from(JpegDecoder::new(&mut img_file)?)?),
        ImageType::UNSUPPORTED => Err(anyhow!(
            "Format of image file {} is not supported. We only support BMP, PNG, JPEG and SVG",
            img_file_name.blue().underline()
        )),
    }
}
