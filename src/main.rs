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
use material::{Dielectric, Lambertian, Metal, RefractiveIndex, Scatter};
use ray::Ray;
use vec::{Color, Point, Vec3};

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

fn random_scene(sample_dist: &Uniform<f64>, rng: &mut ThreadRng) -> hittable::List {
    let mut world = hittable::List::default();
    let ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground));

    for a in 0..20 {
        for b in 0..20 {
            let center = Vec3::new(
                -11.0 + a as f64 + 0.9 * sample_dist.sample(rng),
                0.2,
                -11.0 + b as f64 + 0.9 * sample_dist.sample(rng),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).mag() <= 0.9 {
                continue;
            }

            match sample_dist.sample(rng) {
                choice if choice < 0.8 => {
                    // Matte
                    let random_a = Color::random(sample_dist, rng);
                    let random_b = Color::random(sample_dist, rng);
                    let albedo = random_a.schur(random_b);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
                choice if choice < 0.95 => {
                    // Metal
                    let albedo = Color::new(0.5, 0.5, 0.5) + 0.5 * Color::random(sample_dist, rng);
                    let fuzz = 0.15 * sample_dist.sample(rng);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
                _ => {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(RefractiveIndex::CrownGlass));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let glass = Arc::new(Dielectric::new(RefractiveIndex::CrownGlass));
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, glass));

    let lambert = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, lambert));

    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.3), 0.0));
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, metal));

    world
}

fn main() {
    let height: usize = 1080;
    // TODO: A type for this
    let aspect_ratio = (16, 9);
    let get_width = |height: usize| aspect_ratio.0 * height / aspect_ratio.1;
    let width = get_width(height);
    let samples_per_pixel = 500;
    let max_depth = 50;

    // Random number utilities
    let sample_dist = Uniform::new(0.0, 1.0);
    let vec_dist = Uniform::new_inclusive(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    let mut image_data = vec![0; width * height * 3];

    // Get index of Red value. Green and Blue are +1, +2
    let image_index = |x: usize, y: usize| 3 * (y * width + x);

    let world = random_scene(&sample_dist, &mut rng);

    // Create camera
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(4.0, 1.0, 1.0);
    let up_vector = Vec3::new(0.0, 1.0, 0.0);
    let focal_distance = (look_at - look_from).mag();
    let aperture = 0.1;
    let vertical_fov = 5.0;
    let aspect_ratio = aspect_ratio.0 as f64 / aspect_ratio.1 as f64;

    let camera = Camera::new(
        look_from,
        look_at,
        up_vector,
        vertical_fov,
        aspect_ratio,
        aperture,
        focal_distance,
    );

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

                let r = camera.get_ray(u, v, &vec_dist, &mut rng);
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

    let path = Path::new(r"./output/random-scene-zoom.png");
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
