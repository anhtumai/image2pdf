mod args;
mod image_transform;
mod pagesize;

use args::Args;
use image_transform::create_image_transform;
use pagesize::PageSize;

use std::fs::File;
use std::io::BufWriter;

use clap::Parser;
use image_crate::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use printpdf::{image_crate, Image, ImageTransform, Mm, PdfDocument};

use image_crate::codecs::jpeg::JpegDecoder;

fn main() {
    let args: Args = Args::parse();
    let Args {
        input: input_img_files,
        output: output_pdf_file,
        pagesize,
    } = args;

    if input_img_files.len() == 0 {
        return;
    }

    let page_size = PageSize::new(&pagesize);
    let PageSize { width, height } = page_size;

    let (doc, page1, layer1) =
        PdfDocument::new("Random Document Title", Mm(width), Mm(height), "Layer 1");

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let first_img_file_name = input_img_files.get(0).unwrap();

    let mut img_file = File::open(first_img_file_name).unwrap();

    let img = Image::try_from(JpegDecoder::new(&mut img_file).unwrap()).unwrap();

    let image_transform = create_image_transform(&page_size, &img.image);

    img.add_to_layer(current_layer.clone(), image_transform);

    input_img_files.iter().for_each(|img_file_name| {
        let (next_page, layer1) = doc.add_page(Mm(width), Mm(height), "Layer1");
        let mut img_file = File::open(&img_file_name).unwrap();
        let img = Image::try_from(JpegDecoder::new(&mut img_file).unwrap()).unwrap();

        let image_transform = create_image_transform(&page_size, &img.image);

        let current_layer = doc.get_page(next_page).get_layer(layer1);
        img.add_to_layer(current_layer.clone(), image_transform);
    });

    doc.save(&mut BufWriter::new(File::create(output_pdf_file).unwrap()))
        .unwrap();
}
