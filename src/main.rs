mod math;

use image::{GenericImage, GenericImageView, ImageReader, Rgba, RgbaImage};
use std::{env, println};

use crate::math::rbg_mean;

// TODO handle errors for segments larger than the target image

fn main() {
    let args: Vec<String> = env::args().collect();
    let section_size: u32 = args[1].parse::<u32>().unwrap();

    let img = ImageReader::open("test.png").unwrap();

    let mut decoded = img.decode().unwrap();
    let (width, height) = decoded.dimensions();

    println!("dimensions: {:?}, {:?}", width, height);

    let segment_height = height / section_size;
    let segment_width = width / section_size;

    println!("Segment size: {:?}, {:?}", segment_width, segment_height);
    let mut imgbuf: RgbaImage = image::ImageBuffer::new(width, height);
    for y in (0..height).step_by(segment_height as usize) {
        for x in (0..width).step_by(segment_width as usize) {
            let current_width = segment_width.min(width - x);
            let current_height = segment_height.min(height - y);
            let mut sub_img = decoded.crop(x, y, current_width, current_height);
            let mut pixel_r: Vec<u32> = vec![];
            let mut pixel_g: Vec<u32> = vec![];
            let mut pixel_b: Vec<u32> = vec![];
            let mut pixel_a: Vec<u32> = vec![];
            for pixel in sub_img.pixels() {
                pixel_r.push((pixel.2.0[0] as u32).try_into().unwrap());
                pixel_g.push((pixel.2.0[1] as u32).try_into().unwrap());
                pixel_b.push((pixel.2.0[2] as u32).try_into().unwrap());
                pixel_a.push((pixel.2.0[3] as u32).try_into().unwrap());
            }
            // calculate overall RGBs
            let segment_rgb: Rgba<u8> = image::Rgba([
                rbg_mean(&pixel_r).try_into().unwrap(),
                rbg_mean(&pixel_g).try_into().unwrap(),
                rbg_mean(&pixel_b).try_into().unwrap(),
                rbg_mean(&pixel_a).try_into().unwrap(),
            ]);
            let (sub_width, sub_height) = sub_img.dimensions();

            for y in 0..sub_height {
                for x in 0..sub_width {
                    sub_img.put_pixel(x, y, segment_rgb);
                }
            }

            imgbuf
                .copy_from(&sub_img, x, y)
                .expect("Couldn't copy segment!");
        }
    }

    imgbuf.save("output.png").expect("Couldn't save img!");

    println!("Image size: {}x{}", width, height);
}
