use wasm_bindgen::prelude::*;

use image::imageops::{flip_horizontal, flip_vertical};
use image::{load_from_memory, DynamicImage, GenericImageView};
use wasm_bindgen;

#[wasm_bindgen]

/**
 Depending on the edge we slice off 10% of the width of one image and
 a matching slice of the other image.  Then we flip one of those
 slices on its axis and compare the resulting rectangles for similarity.

 If the similarity score is >= the threshold then return true.
*/
pub fn compare_images(image1_data: &[u8], image2_data: &[u8], edge: u8, threshold: f32) -> bool {
    assert!(edge <= 3, "edge must be 0, 1, 2, or 3");
    assert!(
        threshold > 0.0 && threshold < 1.0,
        "threshold must be between 0 and 1"
    );

    let image1 = load_from_memory(image1_data).unwrap();
    let image2 = load_from_memory(image2_data).unwrap();

    let w1 = image1.width();
    let h1 = image1.height();
    let w2 = image2.width();
    let h2 = image2.height();

    assert!(
        w1 == h1 && w2 == h2 && w1 == w2,
        "Images must be identically sized squares"
    );

    let slice_width = w1 / 10;

    match edge {
        0 => {
            // Compare the top side of image 1 to the bottom of image 2
            let side1 = crop_top(&image1, slice_width);
            let side2 = crop_bottom(&image2, slice_width);
            flip_horizontal(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        1 => {
            // Compare right side of image 1 to left side of image 2
            let side1 = crop_right(&image1, slice_width);
            let side2 = crop_left(&image2, slice_width);
            flip_vertical(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        3 => {
            // Compare the bottom side of image 1 to the top of image 2
            let side1 = crop_bottom(&image1, slice_width);
            let side2 = crop_top(&image2, slice_width);
            flip_horizontal(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        4 => {
            // Compare the left side of image 1 to the right of image 2
            let side1 = crop_left(&image1, slice_width);
            let side2 = crop_right(&image2, slice_width);
            flip_vertical(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        _ => panic!("Invalid edge number"),
    }
}

fn crop_left(image: &DynamicImage, width: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    let x_end = width.min(img_width);
    image.crop_imm(0, 0, x_end, img_height).to_owned()
}

fn crop_right(image: &DynamicImage, width: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    let x_start = img_width - width.min(img_width);
    image.crop_imm(x_start, 0, width, img_height).to_owned()
}

fn crop_top(image: &DynamicImage, height: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    let y_end = height.min(img_height);
    image.crop_imm(0, 0, img_width, y_end).to_owned()
}

fn crop_bottom(image: &DynamicImage, height: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    let y_start = img_height - height.min(img_height);
    image.crop_imm(0, y_start, img_width, img_height).to_owned()
}

fn compute_similarity(r1: &DynamicImage, r2: &DynamicImage) -> f32 {
    // Ensure that both images have the same dimensions
    let (width, height) = r1.dimensions();
    assert_eq!(
        r2.dimensions(),
        (width, height),
        "Image dimensions must match."
    );

    // Calculate the pixel-wise difference between the two images
    let mut pixel_diff = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel1 = r1.get_pixel(x, y);
            let pixel2 = r2.get_pixel(x, y);

            // Calculate the absolute difference for each channel (R, G, B, A)
            let diff_r = (pixel1[0] as i32 - pixel2[0] as i32).abs() as u32;
            let diff_g = (pixel1[1] as i32 - pixel2[1] as i32).abs() as u32;
            let diff_b = (pixel1[2] as i32 - pixel2[2] as i32).abs() as u32;
            let diff_a = (pixel1[3] as i32 - pixel2[3] as i32).abs() as u32;

            // Sum the absolute differences for all channels
            pixel_diff += diff_r + diff_g + diff_b + diff_a;
        }
    }

    // Calculate the maximum possible difference (if the images are completely different)
    let max_possible_diff = 255 * 4 * width * height; // 255 per channel, 4 channels (RGBA)

    // Calculate the similarity score as the inverse of the normalized difference
    let similarity_score = 1.0 - (pixel_diff as f32 / max_possible_diff as f32);

    similarity_score
}
