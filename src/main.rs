use std::fs::File;
use std::io::Write;
use landela_ilanga::structures::vec3::Vec3;

fn main() {
    basic_ppt_image();
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
            output.push_str(&format!("{} {} {}\n",colour.r(),colour.g(),colour.b()));
        }
    }

    let mut file = File::create("./output/image.ppm").unwrap();
    file.write_all(output.as_bytes()).unwrap()
}
