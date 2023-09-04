use wasm_bindgen::prelude::*;

use image::codecs::png::PngEncoder;
use image::imageops::resize;
use image::DynamicImage::ImageRgba8;
use image::{ColorType, GenericImageView};
use image::{ImageBuffer, Rgba};

use wasm_bindgen;

#[wasm_bindgen]
pub fn create_image_grid(
    top_left: &[u8],
    top_middle: &[u8],
    top_right: &[u8],
    middle_left: &[u8],
    middle: &[u8],
    middle_right: &[u8],
    bottom_left: &[u8],
    bottom_middle: &[u8],
    bottom_right: &[u8],
    width: u32,
) -> Vec<u8> {
    let imgs: Vec<_> = [
        top_left,
        top_middle,
        top_right,
        middle_left,
        middle,
        middle_right,
        bottom_left,
        bottom_middle,
        bottom_right,
    ]
    .iter()
    .map(|&data| image::load_from_memory(data).unwrap())
    .collect();

    let individual_width = imgs[0].width();
    let grid_width = individual_width * 3;

    let mut new_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(grid_width, grid_width);

    for (i, img) in imgs.iter().enumerate() {
        let x_offset = (i % 3) as u32 * individual_width;
        let y_offset = (i / 3) as u32 * individual_width;

        for x in 0..individual_width {
            for y in 0..individual_width {
                let pixel = img.get_pixel(x, y);
                new_img.put_pixel(x + x_offset, y + y_offset, pixel);
            }
        }
    }

    let final_image = resize(
        &ImageRgba8(new_img),
        width,
        width,
        image::imageops::FilterType::Lanczos3,
    );

    let mut buf = Vec::new();
    PngEncoder::new(&mut buf)
        .encode(&final_image.into_raw(), width, width, ColorType::Rgba8)
        .unwrap();

    buf
}

#[wasm_bindgen]
pub fn resize_image(image_data: &[u8], width: u32) -> Vec<u8> {
    let img = image::load_from_memory(image_data).unwrap();
    let new_img = resize(&img, width, width, image::imageops::FilterType::Lanczos3);

    let width = new_img.width();
    let height = new_img.height();

    let mut buf = Vec::new();

    PngEncoder::new(&mut buf)
        .encode(&new_img.into_raw(), width, height, ColorType::Rgba8)
        .unwrap();
    buf
}
