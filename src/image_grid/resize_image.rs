use wasm_bindgen::prelude::*;

use image::codecs::png::PngEncoder;
use image::imageops::resize;
use image::load_from_memory;
use image::ColorType;

use wasm_bindgen;

#[wasm_bindgen]
#[allow(deprecated)] // TODO: work out how to fix the `encode` deprecation.
pub fn resize_image(image_data: &[u8], width: u32) -> Vec<u8> {
    let img = load_from_memory(image_data).unwrap();
    let new_img = resize(&img, width, width, image::imageops::FilterType::Lanczos3);

    let width = new_img.width();
    let height = new_img.height();

    let mut buf = Vec::new();

    PngEncoder::new(&mut buf)
        .encode(&new_img.into_raw(), width, height, ColorType::Rgba8)
        .unwrap();
    buf
}
