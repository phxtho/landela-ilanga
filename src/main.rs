use landela_ilanga::materials::{
    lambertian::Lambertian,
    material::{Material, Scatterable},
    metal::Metal,
};
use landela_ilanga::objects::{camera::Camera, sphere::Sphere};
use landela_ilanga::structures::hittable::{HitRecord, Hittable, HittableList};
use landela_ilanga::structures::{ray::Ray, vec3::Vec3};
use landela_ilanga::utils::random_double;

use std::fs::File;
use std::io::Write;

fn main() {
    render_image();
}

fn write_colour(pixel_colour: &Vec3, samples_per_pixel: u32) -> String {
    // Divide the colour by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    let mut colour = *pixel_colour * scale;
    colour = Vec3::new(
        colour.r().sqrt().clamp(0.0, 0.999),
        colour.g().sqrt().clamp(0.0, 0.999),
        colour.b().sqrt().clamp(0.0, 0.999),
    ) * 256.0;
    colour.colourize();
    // Write the translated [0,255] value of each color component
    let output = format!("{} {} {}\n", colour.r(), colour.g(), colour.b());
    return output;
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    // Exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Vec3 = Vec3::default();
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Vec3::new(0., 0., 0.);
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn render_image() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth: u32 = 50;

    // World
    let material_ground = Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.)));
    let material_center = Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.)));
    let material_left = Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.)));
    let material_right = Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.)));

    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);
    let world = HittableList {
        objects: vec![
            Box::new(ground),
            Box::new(sphere_center),
            Box::new(sphere_left),
            Box::new(sphere_right),
        ],
    };

    // Camera
    let cam = Camera::new();

    // Render
    let mut output = format!("P3\n {} {} \n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_colour = Vec3::new(0.0, 0.0, 0.0);
            // Antialiasing: taking multiple samples around the pixel
            for _ in 0..samples_per_pixel {
                let u: f64 = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v: f64 = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_color(&r, &world, max_depth);
            }
            output.push_str(&write_colour(&pixel_colour, samples_per_pixel));
        }
    }

    let mut file = File::create("./output/shiny_metal.ppm").unwrap();
    file.write_all(output.as_bytes()).unwrap()
}
