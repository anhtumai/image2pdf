use printpdf::{scale::Px, xobject::ImageXObject, Mm};

fn pixel_to_mm(Px(pixel): Px) -> Mm {
    Mm((pixel as f64) * 0.084666667)
}

pub fn get_image_dimension_in_mm(image_object: &ImageXObject) -> (Mm, Mm) {
    let ImageXObject { width, height, .. } = image_object;
    (pixel_to_mm(*width), pixel_to_mm(*height))
}
