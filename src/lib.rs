use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine:: general_purpose};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn convert(encoded_file: &str, convert_type: &str) -> String {
    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = match convert_type {
        "grayscale" => img.grayscale(),
        "blur" => img.blur(9.0),
        "flip_vertically" => img.flipv(),
        "flip_horizontally" => img.fliph(),
        "rotate90" => img.rotate90(),
        "rotate180" => img.rotate180(),
        "rotate270" => img.rotate270(),
        _ => img,
    };
     
    log(&"Effect apllied".into());

    let mut buffer =vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new image written ".into());

    let encoded_image = general_purpose::STANDARD.encode(&buffer);
    let data_url = format!("data:image/png;base64,{}",encoded_image);
    
    data_url
}