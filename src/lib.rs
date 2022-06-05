// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
// use base64::decode;
use base64::{ encode , decode };
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn gray_scale(encoded_file: &str) -> String {
    // log(&encoded_file.into());
    log(&"GrayScaled Called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image Decoded".into());

    // let converted_img = load_from_memory(&base64_to_vector).unwrap();
    let mut converted_img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Img Loaded".into());

    converted_img = converted_img.grayscale();
    log(&"GrayScale Effect Applied".into());

    let mut buffer = vec![];
    converted_img.write_to(&mut buffer, Png).unwrap();
    log(&"New Image Written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    data_url
}