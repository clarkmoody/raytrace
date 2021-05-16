use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;

mod camera;
mod hittable;
mod material;
mod ray;
mod vec;

use camera::Camera;
use hittable::{Hittable, Sphere};
use material::{Lambertian, Metal, Scatter};
use ray::Ray;
use vec::{Color, Point};

fn ray_color(
    r: &Ray,
    world: &hittable::List,
    depth: usize,
    vec_dist: &Uniform<f64>,
    rng: &mut ThreadRng,
) -> Color {
    if depth == 0 {
        return Color::ZERO;
    }

    if let Some(hit) = world.hit(r, 0.001..=f64::MAX) {
        if let Some(Scatter { ray, attenuation }) = hit.material.scatter(r, &hit, vec_dist, rng) {
            return attenuation.schur(ray_color(
                &ray,
                world,
                depth.saturating_sub(1),
                vec_dist,
                rng,
            ));
        }
        return Color::ZERO;
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
    let max_depth = 50;

    // Random number utilities
    let sample_dist = Uniform::new(0.0, 1.0);
    let vec_dist = Uniform::new_inclusive(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    let mut image_data = vec![0; width * height * 3];

    // Get index of Red value. Green and Blue are +1, +2
    let image_index = |x: usize, y: usize| 3 * (y * width + x);

    // World materials
    let ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    // World objects
    let mut world = hittable::List::default();
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, center));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left));
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right));

    // Create camera
    let camera = Camera::new(2.0, 16.0 / 9.0, 1.0);

    for y in 0..height {
        print!("\rScanlines remaining {:>5}", height - y);
        std::io::stdout().flush().unwrap();
        for x in 0..width {
            let mut color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let rand_x = sample_dist.sample(&mut rng);
                let rand_y = sample_dist.sample(&mut rng);
                let u = (x as f64 + rand_x) / (width - 1) as f64;
                let v = 1.0 - (y as f64 + rand_y) / (height - 1) as f64;

                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, max_depth, &vec_dist, &mut rng);
            }
            color /= samples_per_pixel as f64;
            // Gamma correction for gamma=2.0 is square root
            color = color.sqrt();

            let rgb: [u8; 3] = color.as_color_u8();
            let i = image_index(x, y);
            image_data[i] = rgb[0];
            image_data[i + 1] = rgb[1];
            image_data[i + 2] = rgb[2];
        }
    }
    print!("\r");

    let path = Path::new(r"./output/shiny-metal.png");
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
