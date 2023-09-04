use wasm_bindgen::prelude::*;

use image::codecs::png::PngEncoder;
use image::imageops::resize;
use image::DynamicImage::ImageRgba8;
use image::{load_from_memory, ImageBuffer, Rgba};
use image::{ColorType, GenericImageView};
use std::cmp::min;

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

// enum Edge {
//     Top,
//     Right,
//     Bottom,
//     Left,
// }

#[wasm_bindgen]
pub fn check_edge_alignment(image1_data: &[u8], image2_data: &[u8], edge: u8) -> bool {
    let img1 = load_from_memory(image1_data).unwrap();
    let img2 = load_from_memory(image2_data).unwrap();

    let width = min(img1.width(), img2.width());
    let height = min(img1.height(), img2.height());

    match edge {
        0 => {
            for x in 0..width {
                if img1.get_pixel(x, 0) != img2.get_pixel(x, height - 1) {
                    return false;
                }
            }
        }
        1 => {
            for y in 0..height {
                if img1.get_pixel(width - 1, y) != img2.get_pixel(0, y) {
                    return false;
                }
            }
        }
        2 => {
            for x in 0..width {
                if img1.get_pixel(x, height - 1) != img2.get_pixel(x, 0) {
                    return false;
                }
            }
        }
        3 => {
            for y in 0..height {
                if img1.get_pixel(0, y) != img2.get_pixel(width - 1, y) {
                    return false;
                }
            }
        }
        _ => {
            return false;
        }
    }

    true
}
