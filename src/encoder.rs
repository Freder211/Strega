
pub fn encode_text(secret: &str, bytes: &Vec<u8>) -> Vec<u8> {
    let secret_bytes = Vec::from(secret.as_bytes());

    let splitted_bits = get_splitted_bits(secret_bytes);
    let mut splitted_bits = add_header(splitted_bits);
    cut_secret_if_too_long(&mut splitted_bits, &bytes);

    let encoded_img_bytes = encode_bits(&splitted_bits, &bytes);

    encoded_img_bytes
}

fn encode_bits(bits_to_encode: &Vec<u8>, source: &Vec<u8>) -> Vec<u8>{
    
    let mut result: Vec<u8> = Vec::with_capacity(source.len());
    for i in 0..bits_to_encode.len() {
        let bit = bits_to_encode[i];
        let byte = source[i];
        result.push(encode_bit_in_byte(bit, byte));
    }
    result.append(&mut source[bits_to_encode.len()..].to_vec());

    result
}

fn add_header(mut bits_to_encode: Vec<u8>) -> Vec<u8> {
    let bits_len = bits_to_encode.len() as u16;
    let first_byte = (bits_len >> 8) as u8;
    let second_byte = (bits_len & 0b0000000011111111) as u8;
    let header = vec![first_byte, second_byte];
    let mut full_data = get_splitted_bits(header);
    full_data.append(&mut bits_to_encode);
    full_data
}

fn cut_secret_if_too_long(secret_bytes: &mut Vec<u8>, img_bytes: &Vec<u8>) {
    
    let secret_len = secret_bytes.len();
    let img_len = img_bytes.len();
    let max_len = img_len + crate::HEADER_SIZE;
    if secret_len * 8 <= max_len {
        return;
    }

    *secret_bytes = secret_bytes[..img_len].to_vec();
}

fn get_splitted_bits(bytes: Vec<u8>) -> Vec<u8> {
    let mut all_splitted_bits: Vec<u8> = Vec::new();
    for b in bytes {
        let mut splitted_bits = get_byte_splitted_bits(b);
        all_splitted_bits.append(&mut splitted_bits);
        
    }
    return all_splitted_bits;
}



fn get_byte_splitted_bits(byte: u8) -> Vec<u8> {
    let mut mask: u8 = 0b10000000;
    let mut splitted_bits: [u8; 8] = [0; 8];
    for i in 0..8 {
        splitted_bits[i] = get_clean_bit(byte & mask);
        mask = mask >> 1;
    }

    return Vec::from(splitted_bits);
}

fn get_clean_bit(value: u8) -> u8 {
    if value > 0 {
        return 0b00000001;
    }
    return 0b000000000;
}

fn encode_bit_in_byte(bit: u8, byte: u8) -> u8 {
    (bit | byte) & (bit | 0b11111110)
}
