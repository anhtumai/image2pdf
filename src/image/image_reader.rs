use std::{fs::File, path::Path};

use anyhow::{anyhow, Result};
use colored::Colorize;
use image_crate::codecs::{bmp::BmpDecoder, jpeg::JpegDecoder, png::PngDecoder};
use printpdf::{
    image_crate::{self, ColorType, ImageDecoder},
    Image,
};

enum ImageType<'a> {
    Bmp,
    Jpeg,
    Png,
    Unsupported(&'a str),
}

fn get_image_type(img_file_name: &str) -> ImageType<'_> {
    let extension_option = img_file_name.split('.').last();
    if let Some(extension) = extension_option {
        return match extension.to_lowercase().as_str() {
            "bmp" => ImageType::Bmp,
            "png" => ImageType::Png,
            "jpg" | "jpeg" => ImageType::Jpeg,
            _ => ImageType::Unsupported(img_file_name),
        };
    }
    ImageType::Unsupported(img_file_name)
}

pub fn read_image_from_file<P: AsRef<Path>>(img_file_name: P) -> Result<(ColorType, Image)> {
    let img_file_name = img_file_name.as_ref();
    let mut img_file = File::open(img_file_name)?;

    match get_image_type(&img_file_name.to_string_lossy()) {
        ImageType::Bmp => {
            let bmp_decoder = BmpDecoder::new(&mut img_file)?;
            let color_type = bmp_decoder.color_type();
            let image = Image::try_from(bmp_decoder)?;
            Ok((color_type, image))
        }
        ImageType::Png => {
            let png_decoder = PngDecoder::new(&mut img_file)?;
            let color_type = png_decoder.color_type();
            let image = Image::try_from(png_decoder)?;
            Ok((color_type, image))
        }
        ImageType::Jpeg => {
            let jpeg_decoder = JpegDecoder::new(&mut img_file)?;
            let color_type = jpeg_decoder.color_type();
            let image = Image::try_from(jpeg_decoder)?;
            Ok((color_type, image))
        }
        ImageType::Unsupported(img_file_name) => Err(anyhow!(
            "Format of image file {} is not supported. We only support BMP, PNG and JPEG",
            img_file_name.blue().underline()
        )),
    }
}
