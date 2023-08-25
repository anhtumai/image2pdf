use std::mem;

use num::{traits::bounds::UpperBounded, FromPrimitive, ToPrimitive};
use printpdf::{image_crate::ColorType, ColorSpace, Image};

fn remove_alpha_from_4_channel<T>(image_data: Vec<T>) -> Vec<T>
where
    T: Clone + ToPrimitive + FromPrimitive + UpperBounded,
{
    let max_value = T::max_value().to_f64().unwrap();
    image_data
        .chunks(4)
        .flat_map(|rgba| {
            let red = rgba[0].to_f64().unwrap();
            let green = rgba[1].to_f64().unwrap();
            let blue = rgba[2].to_f64().unwrap();
            let alpha = rgba[3].to_f64().unwrap() / max_value;
            let new_first = T::from_f64((1.0 - alpha) * max_value + alpha * red).unwrap();
            let new_second = T::from_f64((1.0 - alpha) * max_value + alpha * green).unwrap();
            let new_third = T::from_f64((1.0 - alpha) * max_value + alpha * blue).unwrap();
            [new_first, new_second, new_third]
        })
        .collect()
}

pub trait RemoveAlpha {
    fn remove_alpha(&mut self, color_type: ColorType);
}

impl RemoveAlpha for Image {
    fn remove_alpha(&mut self, color_type: ColorType) {
        match color_type {
            ColorType::Rgba8 => {
                let new_image_data =
                    remove_alpha_from_4_channel(mem::take(&mut self.image.image_data));
                self.image.image_data = new_image_data;
                self.image.color_space = ColorSpace::Rgb;
            }
            ColorType::Rgba16 => {
                let u16_image_data: Vec<u16> = self
                    .image
                    .image_data
                    .chunks(2)
                    .map(|arr| u16::from_be_bytes([arr[0], arr[1]]))
                    .collect();
                let new_u16_image_data = remove_alpha_from_4_channel(u16_image_data);

                let new_u8_image_data = new_u16_image_data
                    .into_iter()
                    .flat_map(u16::to_be_bytes)
                    .collect();

                self.image.image_data = new_u8_image_data;
                self.image.color_space = ColorSpace::Rgb;
            }
            _ => (),
        }
    }
}
