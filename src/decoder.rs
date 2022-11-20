use crate::header::HeaderParser;


pub fn decode_text(bytes: &Vec<u8>) -> String {
    let ls_bits = get_ls_bits(bytes);
    let bytes = restore_bits_to_bytes(&ls_bits);
    let h_parser = HeaderParser::new(&bytes);
    let significant_bytes = h_parser.get_significant_bytes();
    let text = String::from_utf8(significant_bytes).expect("couldn't parse bytes to string");
    text
}


fn get_ls_bits(bytes: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
    let mask: u8 = 0b0000001;
    for b in bytes {
        result.push(b & mask);
    }

    result
}


fn restore_bits_to_bytes(bits: &Vec<u8>) -> Vec<u8> {
    let bytes_len = bits.len() / 8;
    let mut result: Vec<u8> = Vec::with_capacity(bytes_len);
    for i in 0..bytes_len {
        let start = i * 8;
        let end = start + 8;
        let slice: &[u8; 8] = &bits[start..end].try_into().expect("expected 8 lenght array");
        result.push(restore_byte(&slice));

    }

    result
}

fn restore_byte(bits: &[u8; 8]) -> u8 {
    let mut shift_amount: i8 = 7;
    let mut accumolator: u8 = 0;

    for b in bits {
        accumolator = accumolator | (b << shift_amount);
        shift_amount -= 1;
    }

    accumolator
}
