mod encoder;
mod decoder;
mod utils;
mod png_io;

use png_io::{read_image_data, write_image};
use utils::print_vector;

const HEADER_SIZE: usize = 2;

fn main(){
    let source_img =  read_image_data("input.png");
    let encoded_text = encoder::encode_text("miao", &source_img.bytes);
    write_image("out.png", source_img, encoded_text);

    let source_img = read_image_data("output.png");
    let text = decoder::decode_text(&source_img.bytes);
    println!("{text}");

}


