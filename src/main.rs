fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3"); // P3 means colors are ascii
    println!("{image_width} {image_height}");
    println!("255"); // max color

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f32 = i as f32 / (image_width as f32 - 1.0);
            let g: f32 = j as f32 / (image_height as f32 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            println!("{ir} {ig} {ib}");
        }
    }
}
