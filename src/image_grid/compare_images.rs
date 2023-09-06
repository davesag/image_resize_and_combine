use wasm_bindgen::prelude::*;

use image::imageops::{flip_horizontal, flip_vertical};
use image::{load_from_memory, DynamicImage, GenericImageView};
use wasm_bindgen;

#[wasm_bindgen]
pub fn compare_images(image1_data: &[u8], image2_data: &[u8], edge: u8, threshold: f32) -> bool {
    let img1 = load_from_memory(image1_data).unwrap();
    let img2 = load_from_memory(image2_data).unwrap();

    compare_image_sides(&img1, &img2, edge, threshold)
}

fn compare_image_sides(
    image1: &DynamicImage,
    image2: &DynamicImage,
    edge: u8,
    threshold: f32,
) -> bool {
    // Define the width of the side portions to compare (adjust as needed)
    let slice_width = image1.width() / 10;

    match edge {
        0 => {
            // Crop the top side of image 1 to the Bottom of image 2
            let side1 = crop_top(&image1, slice_width);
            let side2 = crop_bottom(&image2, slice_width);
            flip_horizontal(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        1 => {
            // match right side of image 1 with left side of image 2
            let side1 = crop_right(&image1, slice_width);
            let side2 = crop_left(&image2, slice_width);
            flip_vertical(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        3 => {
            // Crop the bottom side of image 1 to the top of image 2
            let side1 = crop_bottom(&image1, slice_width);
            let side2 = crop_top(&image2, slice_width);
            flip_horizontal(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        4 => {
            // Crop the left side of image 1 to the right of image 2
            let side1 = crop_left(&image1, slice_width);
            let side2 = crop_right(&image2, slice_width);
            flip_vertical(&side2);
            return compute_similarity(&side1, &side2) >= threshold;
        }
        _ => panic!("Invalid edge number"),
    }
}

// Function to crop the side of an image (left or right)
fn crop_left(image: &DynamicImage, width: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    let x_end = width.min(img_width);
    image.clone().crop(0, 0, x_end, img_height).to_owned()
}

fn crop_right(image: &DynamicImage, width: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    let x_start = img_width - width.min(img_width);
    image.clone().crop(x_start, 0, width, img_height).to_owned()
}

fn crop_top(image: &DynamicImage, height: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    // Ensure the specified height does not exceed the image's height
    let y_end = height.min(img_height);
    image.clone().crop(0, 0, img_width, y_end).to_owned()
}

fn crop_bottom(image: &DynamicImage, height: u32) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();
    // Ensure the specified height does not exceed the image's height
    let y_start = img_height - height.min(img_height);
    image
        .clone()
        .crop(0, y_start, img_width, img_height)
        .to_owned()
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
