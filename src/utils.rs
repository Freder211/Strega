use image::ImageFormat;



struct FileFormatError {
    detail: String
}

impl FileFormatError {
    fn new(msg: String) -> FileFormatError {
        FileFormatError { detail: msg }
    }
}

pub fn print_vector(vec: &Vec<u8>) {
    for i in &vec[..3] {
        println!("{i:#010b}");
    }
}

pub fn get_image_format_header_byte(format: ImageFormat) -> Result<u8,  FileFormatError> {
    let format_file_type = format.extensions_str()[0];
    for (i, file_type) in crate::SUPPORTED_FILE_FORMATS.iter().enumerate() {
        if &format_file_type == file_type {
            return Ok(i.try_into().unwrap());
        }
    }

    Err(
        FileFormatError::new(
            format!("{format_file_type} could not be found in supported types")
        )
    )
}
