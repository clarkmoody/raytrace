use palette::{Pixel, Srgb};

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod ray;
mod vec;

use ray::Ray;
use vec::{Point, Vec3};

fn ray_color(r: &Ray) -> Srgb {
    // Center of the sphere
    let p = Point::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&p, 0.5, r);
    if t > 0.0 {
        // Unit normal of the sphere
        let n = (r.at(t) - p).unit();
        let color = 0.5 * (n + Vec3::new(1.0, 1.0, 1.0));
        return Srgb::from_components((color.x as f32, color.y as f32, color.z as f32));
    }

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    let color = (1.0 - t) * Point::new(1.0, 1.0, 1.0) + t * Point::new(0.5, 0.7, 1.0);
    Srgb::from_components((color.x as f32, color.y as f32, color.z as f32))
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.mag_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.mag_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    let height: usize = 108;
    // TODO: A type for this
    let aspect_ratio = (16, 9);
    let get_width = |height: usize| aspect_ratio.0 * height / aspect_ratio.1;
    let width = get_width(height);

    let mut image_data = vec![0; width * height * 3];

    // Get index of Red value. Green and Blue are +1, +2
    let image_index = |x: usize, y: usize| 3 * (y * width + x);

    // Camera viewport details
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio.0 as f64 * viewport_height / aspect_ratio.1 as f64;
    let focal_length = 1.0;

    let origin = Point::ZERO;
    // +x to the right
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // +y up
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // +z is out of the frame
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for y in 0..height {
        let v = 1.0 - y as f64 / (height - 1) as f64;

        print!("\rScanlines remaining {:>5}", height - y);
        std::io::stdout().flush().unwrap();

        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let color = ray_color(&r);

            let rgb: [u8; 3] = color.into_format().into_raw();

            let i = image_index(x, y);
            image_data[i] = rgb[0];
            image_data[i + 1] = rgb[1];
            image_data[i + 2] = rgb[2];
        }
    }
    print!("\r");

    let path = Path::new(r"./output/sphere-normals.png");
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
