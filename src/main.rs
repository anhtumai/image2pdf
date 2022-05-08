mod args;
mod image;
mod pagesize;

use clap::Parser;
use colored::*;
use printpdf::{ImageTransform, Mm, PdfDocument};
use std::cmp::max;
use std::fs::File;
use std::io::BufWriter;

use args::Args;
use image::{
    image_object::get_image_dimension_in_mm, image_reader::read_image_from_file,
    image_transform::get_image_transform_for_page_size,
};
use pagesize::PageSizeInMm;

const MIN_WIDTH_IN_MM: f64 = 210.0;
const MIN_HEIGHT_IN_MM: f64 = 297.0;

fn main() {
    let args: Args = Args::parse();
    let Args {
        input: input_img_files,
        output: output_pdf_file,
        pagesize,
    } = args;

    let page_size_option = match pagesize {
        Some(s) => Some(PageSizeInMm::new(&s)),
        None => None,
    };

    let doc = PdfDocument::empty("Random Document Title");

    input_img_files.iter().for_each(|img_file_name| {
        let img_result = read_image_from_file(img_file_name);
        if let Err(ref e) = img_result {
            println!(
                "{}: cannot read file {}. {}: {}",
                "Warning".yellow(),
                img_file_name.blue().underline(),
                "Error".red(),
                e
            );
            return;
        };
        let img = img_result.unwrap();
        if let Some(page_size) = &page_size_option {
            let image_transform = get_image_transform_for_page_size(&page_size, &img.image);
            let PageSizeInMm(width, height) = page_size;
            let (page, layer_index) =
                doc.add_page(Mm(width.to_owned()), Mm(height.to_owned()), "Layer1");
            let current_layer = doc.get_page(page).get_layer(layer_index);
            img.add_to_layer(current_layer.clone(), image_transform);
        } else {
            let (original_image_width, original_image_height) =
                get_image_dimension_in_mm(&img.image);

            let image_scale = max(
                1,
                max(
                    (MIN_WIDTH_IN_MM / original_image_width) as i32,
                    (MIN_HEIGHT_IN_MM / original_image_height) as i32,
                ),
            ) as f64;
            let (page, layer_index) = doc.add_page(
                Mm(original_image_width * image_scale),
                Mm(original_image_height * image_scale),
                "Layer1",
            );
            let current_layer = doc.get_page(page).get_layer(layer_index);
            img.add_to_layer(
                current_layer.clone(),
                ImageTransform {
                    scale_x: Some(image_scale),
                    scale_y: Some(image_scale),
                    ..Default::default()
                },
            );
        };
    });

    doc.save(&mut BufWriter::new(File::create(output_pdf_file).unwrap()))
        .unwrap();
}
