use rand::distributions::{Distribution, Uniform};

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

mod camera;
mod hittable;
mod ray;
mod vec;

use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use vec::{Color, Point, Vec3};

fn ray_color(r: &Ray, world: &hittable::List) -> Color {
    if let Some(hit) = world.hit(r, 0.0..=f64::MAX) {
        return 0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Point::new(1.0, 1.0, 1.0) + t * Point::new(0.5, 0.7, 1.0)
}

fn main() {
    let height: usize = 288;
    // TODO: A type for this
    let aspect_ratio = (16, 9);
    let get_width = |height: usize| aspect_ratio.0 * height / aspect_ratio.1;
    let width = get_width(height);
    let samples_per_pixel = 100;

    // Random number utilities
    let udist = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    let mut image_data = vec![0; width * height * 3];

    // Get index of Red value. Green and Blue are +1, +2
    let image_index = |x: usize, y: usize| 3 * (y * width + x);

    // World objects
    let mut world = hittable::List::default();
    world.add(hittable::Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(hittable::Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // Create camera
    let camera = Camera::new(2.0, 16.0 / 9.0, 1.0);

    for y in 0..height {
        print!("\rScanlines remaining {:>5}", height - y);
        std::io::stdout().flush().unwrap();
        for x in 0..width {
            let mut color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let rand_x = udist.sample(&mut rng);
                let rand_y = udist.sample(&mut rng);
                let u = (x as f64 + rand_x) / (width - 1) as f64;
                let v = 1.0 - (y as f64 + rand_y) / (height - 1) as f64;

                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world);
            }
            color /= samples_per_pixel as f64;

            let rgb: [u8; 3] = color.as_color_u8();
            let i = image_index(x, y);
            image_data[i] = rgb[0];
            image_data[i + 1] = rgb[1];
            image_data[i + 2] = rgb[2];
        }
    }
    print!("\r");

    let path = Path::new(r"./output/antialiasing.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

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
