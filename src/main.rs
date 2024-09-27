use image::{ImageBuffer, Rgb};
use std::env::current_dir;
use thiserror::Error;

#[derive(Error, Debug)]
enum Errors {
    #[error("IO Error")]
    IOError,
}

fn main() {
    // Load the image
    let current_dir = current_dir().map_err(|_| Errors::IOError).unwrap();
    let image_path = current_dir.join("main.png");

    let mut img = image::open(&image_path)
        .expect("Failed to open image")
        .to_rgb8();
    let (_, height) = img.dimensions();

    apply_box_blur(&mut img, 0, height / 3);
    apply_gaussian_blur(&mut img, height / 3, 2 * height / 3);
    add_visual_indicators(&mut img);

    let output_path = current_dir.join("output.png");
    img.save(&output_path).map_err(|_| Errors::IOError).unwrap();
}

fn apply_box_blur(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start_row: u32, end_row: u32) {
    let (width, _) = img.dimensions();
    let kernel_size = 15;
    let mut temp = img.clone();

    for y in start_row..end_row {
        for x in 0..width {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            let mut count = 0;

            for ky in -(kernel_size as i32 / 2)..=(kernel_size as i32 / 2) {
                for kx in -(kernel_size as i32 / 2)..=(kernel_size as i32 / 2) {
                    let nx = x as i32 + kx;
                    let ny = y as i32 + ky;

                    if nx >= 0 && nx < width as i32 && ny >= start_row as i32 && ny < end_row as i32
                    {
                        let pixel = img.get_pixel(nx as u32, ny as u32);
                        sum_r += pixel[0] as u32;
                        sum_g += pixel[1] as u32;
                        sum_b += pixel[2] as u32;
                        count += 1;
                    }
                }
            }

            let blurred_pixel = Rgb([
                (sum_r / count) as u8,
                (sum_g / count) as u8,
                (sum_b / count) as u8,
            ]);
            temp.put_pixel(x, y, blurred_pixel);
        }
    }

    for y in start_row..end_row {
        for x in 0..width {
            img.put_pixel(x, y, *temp.get_pixel(x, y));
        }
    }
}

fn apply_gaussian_blur(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start_row: u32, end_row: u32) {
    let (width, _) = img.dimensions();
    let kernel = [
        [1, 4, 6, 4, 1],
        [4, 16, 24, 16, 4],
        [6, 24, 36, 24, 6],
        [4, 16, 24, 16, 4],
        [1, 4, 6, 4, 1],
    ];
    let kernel_sum: u32 = kernel.iter().flatten().sum();
    let mut temp = img.clone();

    for y in start_row..end_row {
        for x in 0..width {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;

            for ky in 0..5 {
                for kx in 0..5 {
                    let nx = x as i32 + kx as i32 - 2;
                    let ny = y as i32 + ky as i32 - 2;

                    if nx >= 0 && nx < width as i32 && ny >= start_row as i32 && ny < end_row as i32
                    {
                        let pixel = img.get_pixel(nx as u32, ny as u32);
                        let weight = kernel[ky][kx] as u32;
                        sum_r += pixel[0] as u32 * weight;
                        sum_g += pixel[1] as u32 * weight;
                        sum_b += pixel[2] as u32 * weight;
                    }
                }
            }

            let blurred_pixel = Rgb([
                (sum_r / kernel_sum) as u8,
                (sum_g / kernel_sum) as u8,
                (sum_b / kernel_sum) as u8,
            ]);
            temp.put_pixel(x, y, blurred_pixel);
        }
    }

    for y in start_row..end_row {
        for x in 0..width {
            img.put_pixel(x, y, *temp.get_pixel(x, y));
        }
    }
}

fn add_visual_indicators(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (width, height) = img.dimensions();

    // Add red line at 1/3 height
    for x in 0..width {
        img.put_pixel(x, height / 3, Rgb([255, 0, 0]));
    }

    // Add blue line at 2/3 height
    for x in 0..width {
        img.put_pixel(x, 2 * height / 3, Rgb([0, 0, 255]));
    }
}
