use anyhow::{anyhow, Result};
use colored::Colorize;

use std::fs::File;

use image_crate::codecs::{bmp::BmpDecoder, jpeg::JpegDecoder, png::PngDecoder};
use printpdf::{
    image_crate::{self, ColorType, ImageDecoder},
    Image,
};

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

pub fn read_image_from_file(img_file_name: &str) -> Result<(ColorType, Image)> {
    let mut img_file = File::open(&img_file_name)?;

    match get_image_type(img_file_name) {
        ImageType::BMP => {
            let bmp_decoder = BmpDecoder::new(&mut img_file)?;
            let color_type = bmp_decoder.color_type();
            let image = Image::try_from(bmp_decoder)?;
            return Ok((color_type, image));
        }
        ImageType::PNG => {
            let png_decoder = PngDecoder::new(&mut img_file)?;
            let color_type = png_decoder.color_type();
            let image = Image::try_from(png_decoder)?;
            return Ok((color_type, image));
        }
        ImageType::JPEG => {
            let jpeg_decoder = JpegDecoder::new(&mut img_file)?;
            let color_type = jpeg_decoder.color_type();
            let image = Image::try_from(jpeg_decoder)?;
            return Ok((color_type, image));
        }
        ImageType::UNSUPPORTED => Err(anyhow!(
            "Format of image file {} is not supported. We only support BMP, PNG, JPEG and SVG",
            img_file_name.blue().underline()
        )),
    }
}
