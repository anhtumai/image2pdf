use printpdf::{xobject::ImageXObject, ImageTransform, Mm};

use crate::pagesize::PageSizeInMm;

use crate::image::image_x_object::get_image_dimension_in_mm;

pub fn get_image_transform_for_page_size(
    page_size: &PageSizeInMm,
    image_object: &ImageXObject,
) -> ImageTransform {
    let PageSizeInMm(page_size_width, page_size_height) = page_size;
    let page_size_ratio = page_size_width / page_size_height;

    let (image_width, image_height) = get_image_dimension_in_mm(image_object);
    let image_object_ratio: f64 = image_width / image_height;

    if page_size_ratio > image_object_ratio {
        let scale_based_on_height = page_size_height / image_height;
        let new_image_width = image_width * scale_based_on_height;
        return ImageTransform {
            translate_x: Some(Mm((page_size_width - new_image_width) / 2.0)),
            translate_y: Some(Mm(0.0)),
            rotate: None,
            scale_x: Some(scale_based_on_height),
            scale_y: Some(scale_based_on_height),
            dpi: Some(300.0),
        };
    }

    let scale_based_on_width = page_size_width / image_width;
    let new_image_height = image_height * scale_based_on_width;
    ImageTransform {
        translate_x: Some(Mm(0.0)),
        translate_y: Some(Mm((page_size_height - new_image_height) / 2.0)),
        rotate: None,
        scale_x: Some(scale_based_on_width),
        scale_y: Some(scale_based_on_width),
        dpi: Some(300.0),
    }
}
