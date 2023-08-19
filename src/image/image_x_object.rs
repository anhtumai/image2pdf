use printpdf::{scale::Px, xobject::ImageXObject, Mm};

fn pixel_to_mm(pixel: usize) -> Mm {
    Mm((pixel as f64) * 0.084666667)
}

pub fn get_image_dimension_in_mm(image_object: &ImageXObject) -> (Mm, Mm) {
    let ImageXObject {
        width: Px(image_width_in_px),
        height: Px(image_height_in_px),
        ..
    } = image_object;

    let image_width = pixel_to_mm(*image_width_in_px);
    let image_height = pixel_to_mm(*image_height_in_px);
    (image_width, image_height)
}
