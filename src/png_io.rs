use std::io::{Cursor, Read, Seek, SeekFrom};

use std::{path::Path};
use std::io::{BufWriter};
use bytes::{Bytes, BufMut, Buf};
use image::{ColorType, ImageFormat, DynamicImage};
use std::fs::File;


pub struct ImageData {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub format: ImageFormat
}

pub fn read_image_data_from_bytes(bytes: Vec<u8>, format: ImageFormat) -> ImageData {
    let img = image::load_from_memory_with_format(&bytes, format).unwrap();
    return read_image_data(img, format);
}


pub fn read_image_data_from_file(image_path: &str) -> ImageData {
    let img = image::open(image_path).unwrap();
    let format = ImageFormat::from_path(image_path).unwrap();
    return read_image_data(img, format);
}

fn read_image_data(img: DynamicImage, format: ImageFormat) -> ImageData {
    let width = img.width();
    let height = img.height();
    let color_type = img.color();
    let bytes = img.into_bytes();
    return ImageData {
        bytes,
        width,
        height,
        color_type,
        format
    }
}

pub fn write_image_to_bytes(source_img: ImageData, encoded_bytes: Vec<u8>, format: ImageFormat) -> Vec<u8> {
    // using Cursor instead of a regular Vec because it is required by the library
    let mut writer = Cursor::new(Vec::with_capacity(encoded_bytes.len()));
    {
        let mut buf = BufWriter::with_capacity(encoded_bytes.len(), &mut writer);
        image::write_buffer_with_format(&mut buf,
                                        &encoded_bytes,
                                        source_img.width,
                                        source_img.height,
                                        source_img.color_type,
                                        format).unwrap();
    }

    // converting Cursor back into a Vec
    writer.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    writer.read_to_end(&mut out).unwrap();

    return out;
}


pub fn write_image(output_path: &str, source_img: ImageData, encoded_bytes: Vec<u8>) {
    image::save_buffer(output_path, &encoded_bytes, source_img.width, source_img.height, source_img.color_type).unwrap();
}

