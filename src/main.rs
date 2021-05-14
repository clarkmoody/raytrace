use palette::{Pixel, Srgb};

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let height: usize = 288;
    // TODO: A type for this
    let aspect_ratio = (16, 9);
    let get_width = |height: usize| aspect_ratio.0 * height / aspect_ratio.1;
    let width = get_width(height);

    let mut image_data = vec![0; width * height * 3];

    // Get index of Red value. Green and Blue are +1, +2
    let image_index = |x: usize, y: usize| 3 * (y * width + x);

    for y in 0..height {
        let pct_y = 1.0 - y as f32 / height as f32;
        for x in 0..width {
            let pct_x = x as f32 / width as f32;

            let color = Srgb::from_components((pct_x, pct_y, 0.25));
            let rgb: [u8; 3] = color.into_format().into_raw();

            let i = image_index(x, y);
            image_data[i] = rgb[0];
            image_data[i + 1] = rgb[1];
            image_data[i + 2] = rgb[2];
        }
    }

    let path = Path::new(r"./output/colors.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    match writer.write_image_data(&image_data) {
        Ok(()) => {
            println!("Wrote image data. {} x {}", width, height);
        }
        Err(err) => {
            println!("Image writer error: {:?}", err);
        }
    }
}
