use dotenv::dotenv;
use ray_tracer::{Color, Vec3};
use std::{fs::File, sync::Arc};
use tracing::{debug, trace, Level};

#[tracing::instrument(ret)]

fn main() {
    trace_setup();
    let image_width = 256;
    let image_height = 256;

    println!("P3"); // P3 means colors are ascii
    println!("{image_width} {image_height}");
    println!("255"); // max color
    for j in 0..image_height {
        //trace!("Scanlines remaining {}", image_height - j);
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width as f64 - 1.0);
            let g: f64 = j as f64 / (image_height as f64 - 1.0);
            let b = 0.0;
            let pixel_color = Color::new(r, g, b);
            pixel_color.write_color();
        }
    }
}

fn trace_setup() {
    dotenv().ok();

    let log_file = File::create("debug.log");
    let debug_log = tracing_subscriber::fmt()
        .with_writer(Arc::new(log_file.unwrap()))
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(debug_log).unwrap();
}
