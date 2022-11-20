mod encoder;
mod decoder;
mod utils;
mod png_io;

use utils::print_vector;

use crate::png_io::{read_image_data_from_file, write_image};

const HEADER_SIZE: usize = 2;

fn main(){
    let source_img =  read_image_data_from_file("input.png");
    let encoded_text = encoder::encode_text("miaoaaa", &source_img.bytes);
    write_image("output.png", source_img, encoded_text);

    let source_img = read_image_data_from_file("output.png");
    let text = decoder::decode_text(&source_img.bytes);
    println!("ehi");
    println!("{text}");

}


