use image::ImageFormat;

pub const HEADER_SIZE: usize = 2 + 1;
pub const SUPPORTED_FILE_FORMATS: &[&str] = &[
    "avif",
    "jpg",
    "jpeg",
    "png",
    "gif",
    "webp",
    "tiff",
    "tif",
    "tga",
    "dds",
    "bmp",
    "ico",
    "hdr",
    "exr",
    "pbm",
    "pam",
    "ppm",
    "pgm",
    "ff"
];



pub struct HeaderParser<'a> {
    bytes: &'a Vec<u8>
}

impl<'a> HeaderParser<'a> {
    pub fn new(bytes: &Vec<u8>) -> HeaderParser {
        HeaderParser { bytes }
    }

    pub fn get_amount_significant_bytes(&self) -> u16 {
        let mut accumolator;
        let first_byte = self.bytes[0];
        let second_byte = self.bytes[1];
        accumolator = (first_byte as u16) << 8;
        accumolator = accumolator | second_byte as u16;
        // divide by eight because the header stores the number of bits (we want bytes)
        accumolator / 8 
    }

    pub fn get_significant_bytes(self) -> Vec<u8> {
        let start = HEADER_SIZE;
        let length: usize = self.get_amount_significant_bytes().try_into().unwrap(); 
        let end = start + length;
        return Vec::from(&self.bytes[start..end]);
    }

    pub fn get_format(self) -> ImageFormat {
        let format_byte: usize = self.bytes[2].try_into().unwrap();
        let format = SUPPORTED_FILE_FORMATS[format_byte];
        ImageFormat::from_extension(format).unwrap()
    }
}
