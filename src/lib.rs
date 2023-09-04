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

#[wasm_bindgen]
pub fn check_edge_alignment(
    image1_data: &[u8],
    image2_data: &[u8],
    edge: u8,
    fuzziness: u8,
) -> bool {
    let img1 = load_from_memory(image1_data).unwrap();
    let img2 = load_from_memory(image2_data).unwrap();

    let width = min(img1.width(), img2.width());
    let height = min(img1.height(), img2.height());

    fn is_similar(a: u8, b: u8, fuzziness: u8) -> bool {
        (a as i16 - b as i16).abs() <= fuzziness as i16
    }

    fn pixels_similar(px1: &Rgba<u8>, px2: &Rgba<u8>, fuzziness: u8) -> bool {
        is_similar(px1[0], px2[0], fuzziness)
            && is_similar(px1[1], px2[1], fuzziness)
            && is_similar(px1[2], px2[2], fuzziness)
            && is_similar(px1[3], px2[3], fuzziness)
    }

    let comparator = |x1, y1, x2, y2| {
        let pixel1 = img1.get_pixel(x1, y1);
        let pixel2 = img2.get_pixel(x2, y2);
        pixels_similar(&pixel1, &pixel2, fuzziness)
    };

    match edge {
        0 => {
            // Top
            for x in 0..width {
                if !comparator(x, 0, x, height - 1) {
                    return false;
                }
            }
        }
        1 => {
            // Right
            for y in 0..height {
                if !comparator(width - 1, y, 0, y) {
                    return false;
                }
            }
        }
        2 => {
            // Bottom
            for x in 0..width {
                if !comparator(x, height - 1, x, 0) {
                    return false;
                }
            }
        }
        3 => {
            // Left
            for y in 0..height {
                if !comparator(0, y, width - 1, y) {
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
