use std::fs::File;
use std::io::Write;

fn main() {
    // Image
    let  image_width:u32 = 256;
    let  image_height:u32 = 256;

    // Render
    let mut output = format!("P3\n {} {} \n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f64  = i as f64/((image_width-1) as f64);
            let g: f64 = j as f64/((image_height-1) as f64);
            let b: f64 =  0.25;

            let ir = (r * 255.999) as u32;
            let ig = (g * 255.999) as u32;
            let ib = (b * 255.999) as u32;
            output.push_str(&format!("{} {} {}\n",ir,ig,ib));
        }
    }

    let mut file = File::create("./output/image.ppm").unwrap();
    file.write_all(output.as_bytes()).unwrap()
}
