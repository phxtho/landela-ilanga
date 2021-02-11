use std::fs::File;
use std::io::Write;
use landela_ilanga::structures::{vec3::Vec3,ray::Ray};
use landela_ilanga::structures::hittable::{HittableList, Hittable, HitRecord};
use landela_ilanga::objects::sphere::Sphere;

fn main() {
    gradient_image();
}

fn write_colour (vector : & Vec3) -> String {
    let colour = *vector * 255.99;
    let output = format!("{} {} {}\n",colour.r().floor(),colour.g().floor(),colour.b().floor()); 
    return output;
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

fn hit_sphere(center: &Vec3, radius :f64, r:&Ray) -> f64 {
    // solve quadratic formula for (P(t)−C)⋅(P(t)−C)=r^2
    let oc : Vec3 = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b * half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    }else {
        return (-half_b - discriminant.sqrt())/(2.0 * a);
    } 
}

fn ray_color(r : &Ray, world : &dyn Hittable) -> Vec3{
    let mut rec = HitRecord::new();
    
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::new(1.0,1.0,1.0))
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 -t) * Vec3::new(1.0,1.0,1.0) + t * Vec3::new(0.5,0.7,1.0);
}

fn gradient_image() {
    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width : u32 = 400;
    let image_height : u32 = (image_width as f64/aspect_ratio) as u32 ; 

    // World
    let sphere_a = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_b = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = HittableList {    
        objects: vec![Box::new(sphere_a), Box::new(sphere_b)]
    }; 

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
            let pixel_colour = ray_color(&r, &world);
            output.push_str(&write_colour(&pixel_colour));
        }
    }

    let mut file = File::create("./output/sphere_with_ground.ppm").unwrap();
    file.write_all(output.as_bytes()).unwrap()
}