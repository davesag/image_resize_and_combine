use wasm_bindgen::prelude::*;

use image::GenericImageView;
use image::{load_from_memory, Rgba};
use std::cmp::min;

use wasm_bindgen;

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
        let actual = (a as i16 - b as i16).abs();

        actual <= fuzziness as i16
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
