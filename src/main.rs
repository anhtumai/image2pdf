mod args;
mod image;
mod pagesize;

use args::Args;
use image::{
    image_object::get_image_dimension_in_mm, image_reader::read_image_from_file,
    image_transform::get_image_transform_for_page_size,
};
use pagesize::PageSize;

use std::fs::File;
use std::io::BufWriter;

use clap::Parser;
use printpdf::{ImageTransform, Mm, PdfDocument};

fn main() {
    let args: Args = Args::parse();
    let Args {
        input: input_img_files,
        output: output_pdf_file,
        pagesize,
    } = args;

    let page_size_option = match pagesize {
        Some(s) => Some(PageSize::new(&s)),
        None => None,
    };

    let doc = PdfDocument::empty("Random Document Title");

    input_img_files.iter().for_each(|img_file_name| {
        let img_result = read_image_from_file(img_file_name);
        if let Err(ref e) = img_result {
            println!("Warning: cannot read file {}. Error: {}", img_file_name, e);
            return;
        };
        let img = img_result.unwrap();
        if let Some(page_size) = &page_size_option {
            let image_transform = get_image_transform_for_page_size(&page_size, &img.image);
            let PageSize { width, height } = page_size;
            let (page, layer1) =
                doc.add_page(Mm(width.to_owned()), Mm(height.to_owned()), "Layer1");
            let current_layer = doc.get_page(page).get_layer(layer1);
            img.add_to_layer(current_layer.clone(), image_transform);
        } else {
            let (image_width, image_height) = get_image_dimension_in_mm(&img.image);
            let (page, layer_index) = doc.add_page(Mm(image_width), Mm(image_height), "Layer1");
            let current_layer = doc.get_page(page).get_layer(layer_index);
            img.add_to_layer(current_layer.clone(), ImageTransform::default());
        };
    });

    doc.save(&mut BufWriter::new(File::create(output_pdf_file).unwrap()))
        .unwrap();
}
