use std::{path::Path, io::Write};
use std::io::{BufWriter};
use bytes::{Bytes, BufMut, Buf};
use image::ColorType;
use std::fs::File;


pub struct ImageData {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType
}

pub fn read_image_data_from_bytes(bytes: Vec<u8>) -> ImageData {

    return ImageData {
        bytes: Vec::from([]),
        width: 1,
        height: 1,
        color_type: ColorType::La16
    }
}

pub fn read_image_data(image_path: &str) -> ImageData {
    let image = image::open(image_path).unwrap();
    let width = image.width();
    let height = image.height();
    let color_type = image.color();
    let bytes = image.into_bytes();
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

