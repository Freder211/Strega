use wasm_bindgen::prelude::*;

mod encoder;
mod decoder;
mod utils;
mod png_io;

use png_io::{read_image_data, write_image, read_image_data_from_bytes, write_image_to_bytes};
use utils::print_vector;

const HEADER_SIZE: usize = 2;




// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_array(a: &[u8]);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn test_gatto() {
    alert("micio miao");
}

fn main(){
    let source_img =  read_image_data("boh.png");
    let encoded_text = encoder::encode_text("miao", &source_img.bytes);
    write_image("out.png", source_img, encoded_text);

    let source_img = read_image_data("out.png");
    let text = decoder::decode_text(&source_img.bytes);
    println!("{text}");

}


pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn encode_file(bytes: Vec<u8>, text: &str) -> Vec<u8> {
    set_panic_hook();

    let source_img =  read_image_data_from_bytes(bytes);
    let encoded_text = encoder::encode_text(text, &source_img.bytes);
    write_image_to_bytes(source_img, encoded_text)
}

#[wasm_bindgen]
pub fn decode_file(bytes: Vec<u8>) -> String {
    set_panic_hook();

    let source_img =  read_image_data_from_bytes(bytes);
    let decoded_text = decoder::decode_text(&source_img.bytes);
    decoded_text
}
