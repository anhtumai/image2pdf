use printpdf::{image_crate::ColorType, ColorSpace, Image};

fn remove_alpha_from_4_channel<T>(image_data: &Vec<T>, max_value: f64) -> Vec<T>
where
    T: Clone + num::ToPrimitive + num::FromPrimitive,
{
    let new_image_data = image_data
        .chunks(4)
        .map(|rgba| {
            let first = rgba[0].to_f64().unwrap();
            let second = rgba[1].to_f64().unwrap();
            let third = rgba[2].to_f64().unwrap();
            let alpha = rgba[3].to_f64().unwrap() / max_value;
            let new_first = T::from_f64((1.0 - alpha) * max_value + alpha * first).unwrap();
            let new_second = T::from_f64((1.0 - alpha) * max_value + alpha * second).unwrap();
            let new_third = T::from_f64((1.0 - alpha) * max_value + alpha * third).unwrap();
            return [new_first, new_second, new_third];
        })
        .collect::<Vec<[T; 3]>>()
        .concat();
    return new_image_data;
}

pub trait RemoveAlpha {
    fn remove_alpha(&mut self, color_type: ColorType);
}

impl RemoveAlpha for Image {
    fn remove_alpha(&mut self, color_type: ColorType) {
        use ColorType::*;
        match color_type {
            Rgba8 | Bgra8 => {
                let new_image_data =
                    remove_alpha_from_4_channel(&self.image.image_data, u8::MAX as f64);
                self.image.image_data = new_image_data;
                self.image.color_space = ColorSpace::Rgb;
            }
            Rgba16 => {
                let u16_image_data: Vec<u16> = self
                    .image
                    .image_data
                    .chunks(2)
                    .map(|arr| {
                        let x1 = arr[0];
                        let x2 = arr[1];
                        return (x1 as u16) * 256 + (x2 as u16);
                    })
                    .collect();
                let new_u16_image_data =
                    remove_alpha_from_4_channel(&u16_image_data, u16::MAX as f64);

                let new_u8_image_data = new_u16_image_data
                    .into_iter()
                    .map(|num| [(num / 256) as u8, (num % 256) as u8])
                    .collect::<Vec<[u8; 2]>>()
                    .concat();

                self.image.image_data = new_u8_image_data;
                self.image.color_space = ColorSpace::Rgb;
            }
            _ => return,
        }
    }
}
