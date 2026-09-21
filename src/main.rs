mod math;

use image::{GenericImage, GenericImageView, ImageReader, Rgba, RgbaImage};
use std::println;

use crate::math::rbg_mean;

type Coords = [u128; 2];

fn main() {
    const SECTION_NUM: u32 = 180;

    let img = ImageReader::open("test.png").unwrap();

    let mut decoded = img.decode().unwrap();
    let (width, height) = decoded.dimensions();

    println!("dimensions: {:?}, {:?}", width, height);

    let segment_size = (width * height) / SECTION_NUM;

    println!("Segment size: {:?}", segment_size);
    let mut imgbuf: RgbaImage = image::ImageBuffer::new(width, height);
    let segments: Vec<(u32, u32, u32, Rgba<u8>)> = vec![];
    let rows: u32 = height / segment_size;
    let cols: u32 = width / segment_size;
    for ci in 0..cols {
        for i in 0..rows {
            //let mut pixel_rgbs: Vec<Rgba<u32>> = vec![];

            let sub_img = &decoded.crop(
                i * segment_size,
                ci * segment_size,
                segment_size,
                segment_size,
            );
            let mut pixel_r: Vec<u32> = vec![];
            let mut pixel_g: Vec<u32> = vec![];
            let mut pixel_b: Vec<u32> = vec![];
            let mut pixel_a: Vec<u32> = vec![];
            for (idx, pixel) in sub_img.pixels().enumerate() {
                println!("Calulating pixel no. {:?}...", idx);
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
            println!("RGBA value of segment {:?} is {:?}", i, segment_rgb);
            //sub_img.put_pixel(x, y, pixel);
        }
    }

    for (idx, pixel) in decoded.pixels().enumerate() {
        let seg_num_x = pixel.0 / segment_size;
        let seg_num_y = pixel.1 / segment_size;

        println!("{:?}", pixel);
    }
    println!("image size: {}x{}", width, height);
}
