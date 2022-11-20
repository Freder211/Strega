mod encoder;
mod decoder;
mod utils;
mod png_io;
mod header;

use image::ImageFormat;
use utils::print_vector;

use crate::png_io::{read_image_data_from_file, write_image};


fn main(){
    let input_file = &"input.png";
    let source_img =  read_image_data_from_file(input_file);
    let format = ImageFormat::from_path(input_file).unwrap();
    let encoded_text = encoder::encode_text("ciao mi chiamo Federico", &source_img.bytes, format);
    write_image("output.png", source_img, encoded_text);

    let source_img = read_image_data_from_file("output.png");
    let text = decoder::decode_text(&source_img.bytes);
    println!("ehi");
    println!("{text}");

}


