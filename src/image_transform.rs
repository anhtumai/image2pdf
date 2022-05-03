use crate::pagesize::PageSize;
use printpdf::{scale::Px, xobject::ImageXObject, ImageTransform, Mm};

fn pixel_to_mm(pixel: usize) -> f64 {
    (pixel as f64) * 0.084666667
}

pub fn create_image_transform(page_size: &PageSize, image_object: &ImageXObject) -> ImageTransform {
    let PageSize {
        width: page_size_width,
        height: page_size_height,
    } = page_size;
    let page_size_ratio = page_size_width / page_size_height;

    let ImageXObject {
        width: Px(image_width_in_px),
        height: Px(image_height_in_px),
        ..
    } = image_object;

    let image_width = pixel_to_mm(*image_width_in_px);
    let image_height = pixel_to_mm(*image_height_in_px);

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
