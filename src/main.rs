use std::fs::File;
use std::io::Write;
use landela_ilanga::structures::vec3::Vec3;
use landela_ilanga::structures::ray::Ray;

fn main() {
    gradient_image();
}

fn basic_ppt_image() {
    // Image
    let  image_width:u32 = 256;
    let  image_height:u32 = 256;

    // Render
    let mut output = format!("P3\n {} {} \n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let mut colour = Vec3::new(i as f64/((image_width-1) as f64), j as f64/((image_height-1) as f64), 0.25 ) * 255.999 ;
            colour.colourize();
            output.push_str(&write_colour(&colour));
        }
    }

    let mut file = File::create("./output/image.ppm").unwrap();
    file.write_all(output.as_bytes()).unwrap()
}

fn ray_color(r : &Ray) -> Vec3{
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 -t) * Vec3::new(1.0,1.0,1.0) + t * Vec3::new(0.5,0.7,1.0);
}

fn write_colour (vector : & Vec3) -> String {
    let colour = *vector * 255.99;
    let output = format!("{} {} {}\n",colour.r().floor(),colour.g().floor(),colour.b().floor()); 
    return output;
}

fn gradient_image() {
    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width : u32 = 400;
    let image_height : u32 = image_width/(aspect_ratio as u32) ; 

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0, focal_length);

    // Render
    let mut output = format!("P3\n {} {} \n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u : f64 = i as f64/(image_width-1) as f64;
            let v : f64 = j as f64 /(image_height-1) as f64;
            let r = Ray::new(origin,lower_left_corner + (u as f64 * horizontal) + (v as f64 * vertical) - origin);
            let pixel_colour = ray_color(&r);
            output.push_str(&write_colour(&pixel_colour));
        }
    }

    let mut file = File::create("./output/ray_image.ppm").unwrap();
    file.write_all(output.as_bytes()).unwrap()
}