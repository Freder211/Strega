mod encoder;
mod decoder;
mod utils;
mod png_io;
mod header;

use std::{path::Path, fmt::format};
use image::ImageFormat;
use utils::print_vector;

use crate::png_io::{read_image_data_from_file, write_image};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   input: String,
   text: String,
   #[arg(short, long)]
   output: Option<String>,
}


fn main(){
    let args = Args::parse();
    let input = args.input;
    let output;
    if let Some(specified_output) = args.output.as_deref() {
        output = String::from(specified_output);
    }
    else {
        let input_path = Path::new(&input);
        output = get_output_filename(input_path);
    }
    let source_img =  read_image_data_from_file(&input);
    let format = ImageFormat::from_path(&input).unwrap();
    let encoded_text = encoder::encode_text(&args.text, &source_img.bytes, format);
    write_image(&output, source_img, encoded_text);

    let source_img = read_image_data_from_file(&output);
    let out_text = decoder::decode_text(&source_img.bytes);
    println!("{out_text}");
}



fn get_output_filename(input: &Path) -> String {
    let file_name = input.file_stem().unwrap().to_str().unwrap();
    let ext = input.extension().unwrap().to_str().unwrap();

    return format!("{file_name}-out.{ext}");
}
