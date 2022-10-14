use std::{path::Path, io::Write};
use std::io::{BufWriter};
use bytes::{Bytes, BufMut, Buf};
use png::{Decoder, ColorType};
use std::fs::File;


pub struct ImageData {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType
}

pub fn read_image_data_from_bytes(bytes: Vec<u8>) -> ImageData {
    let reader = bytes.reader();
    let decoder = Decoder::new(reader);
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = Vec::from(&buf[..info.buffer_size()]);
    return ImageData {
        bytes,
        width: info.width,
        height: info.height,
        color_type: info.color_type,
    };
}

pub fn read_image_data(image_path: &str) -> ImageData {
    let file = File::open(image_path).unwrap();
    let decoder = Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = Vec::from(&buf[..info.buffer_size()]);
    return ImageData {
        bytes,
        width: info.width,
        height: info.height,
        color_type: info.color_type,
    };
}

pub fn write_image_to_bytes(source_img: ImageData, encoded_bytes: Vec<u8>) -> Vec<u8> {
    let mut dest_bytes = vec![].writer();
    {
        let mut encoder = png::Encoder::new(&mut dest_bytes, source_img.width, source_img.height); 
        encoder.set_color(png::ColorType::Rgb);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&encoded_bytes).unwrap();
    }
    dest_bytes.into_inner()
}


pub fn write_image(output_path: &str, source_img: ImageData, encoded_bytes: Vec<u8>) {
    let path = Path::new(output_path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, source_img.width, source_img.height); 
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&encoded_bytes).unwrap();
}

