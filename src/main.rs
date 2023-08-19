mod args;
mod image;
mod pagesize;

use anyhow::anyhow;
use clap::Parser;
use colored::*;
use printpdf::{ImageTransform, Mm, PdfDocument};
use std::cmp::max;
use std::fs::File;
use std::io::BufWriter;

use args::Args;
use image::{
    alpha_remover::RemoveAlpha, image_reader::read_image_from_file,
    image_transform::get_image_transform_for_page_size, image_x_object::get_image_dimension_in_mm,
};
use pagesize::PageSizeInMm;

const MIN_WIDTH_IN_MM: Mm = Mm(210.0);
const MIN_HEIGHT_IN_MM: Mm = Mm(297.0);

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    println!("{:?}", args);
    let Args {
        input: input_img_files,
        output: output_pdf_file,
        pagesize: pagesize_option,
    } = args;

    let doc = PdfDocument::empty("Random Document Title");

    for img_file_name in input_img_files {
        let (color_type, mut img) = read_image_from_file(&img_file_name).map_err(|e| {
            anyhow!(
                "{}: cannot read file {}. {}: {}",
                "Warning".yellow(),
                img_file_name.to_string_lossy().blue().underline(),
                "Error".red(),
                e
            )
        })?;
        match pagesize_option {
            Some(pagesize @ PageSizeInMm { width, height }) => {
                let image_transform = get_image_transform_for_page_size(pagesize, &img.image);
                let (page, layer_index) = doc.add_page(width, height, "Layer1");
                let current_layer = doc.get_page(page).get_layer(layer_index);
                img.remove_alpha(color_type);
                img.add_to_layer(current_layer.clone(), image_transform);
            }
            None => {
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
                    original_image_width * image_scale,
                    original_image_height * image_scale,
                    "Layer1",
                );
                let current_layer = doc.get_page(page).get_layer(layer_index);
                img.remove_alpha(color_type);
                img.add_to_layer(
                    current_layer.clone(),
                    ImageTransform {
                        scale_x: Some(image_scale),
                        scale_y: Some(image_scale),
                        ..Default::default()
                    },
                );
            }
        }
    }

    doc.save(&mut BufWriter::new(File::create(output_pdf_file)?))?;
    Ok(())
}
