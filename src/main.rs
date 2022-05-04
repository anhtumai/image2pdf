mod args;
mod image_transform;
mod pagesize;

use args::Args;
use image_transform::{create_image_transform, get_image_dimension_in_mm};
use pagesize::PageSize;

use std::fs::File;
use std::io::BufWriter;

use clap::Parser;
use printpdf::{image_crate, Image, ImageTransform, Mm, PdfDocument};

use image_crate::codecs::jpeg::JpegDecoder;

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
        let mut img_file = File::open(&img_file_name).unwrap();
        let img = Image::try_from(JpegDecoder::new(&mut img_file).unwrap()).unwrap();
        if let Some(page_size) = &page_size_option {
            let image_transform = create_image_transform(&page_size, &img.image);
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
