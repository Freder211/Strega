use std::{path::Path, io::Write};
use std::io::{BufWriter};
use bytes::{Bytes, BufMut, Buf};
use image::{ColorType, ImageFormat, DynamicImage};
use std::fs::File;


pub struct ImageData {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType
}

pub fn read_image_data_from_bytes(bytes: Vec<u8>, format: ImageFormat) -> ImageData {

    let img = image::load_from_memory_with_format(&bytes, format).unwrap();
    return read_image_data(img);
}


pub fn read_image_data_from_file(image_path: &str) -> ImageData {
    let img = image::open(image_path).unwrap();
    return read_image_data(img);
}

fn read_image_data(img: DynamicImage) -> ImageData {
    let width = img.width();
    let height = img.height();
    let color_type = img.color();
    let bytes = img.into_bytes();
    return ImageData {
        bytes,
        width,
        height,
        color_type
    }
}

pub fn write_image_to_bytes(source_img: ImageData, encoded_bytes: Vec<u8>) -> Vec<u8> {
    return Vec::from([]);
}


pub fn write_image(output_path: &str, source_img: ImageData, encoded_bytes: Vec<u8>) {
    image::save_buffer(output_path, &encoded_bytes, source_img.width, source_img.height, source_img.color_type).unwrap();
}

