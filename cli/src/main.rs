use dotenv::dotenv;
use ray_tracer::point3::Point3;
use ray_tracer::ray::Ray;
use ray_tracer::vec3::Vec3;
use std::{fs::File, sync::Arc};
use tracing::{debug, trace, Level};

#[tracing::instrument(ret)]

fn main() {
    trace_setup();

    // ------------Image Setup------------

    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0; // Width / height

    // Setting height based on width and aspect ratio
    // Must be at least 1
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let image_height = if image_height > 1 { image_height } else { 1 };

    // ------------Camera Setup------------

    // Creating the viewport (rectangle in the 3d world which contains pixel grid)
    let viewport_height = 2.0;
    // We cannot use aspect_ratio for this due to it being the ideal value
    // and not necessarily the real value
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // Focal length is distance between camera center and viewport
    let focal_length = 1.0;

    // Point where all rays originate
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Vector across x axis of viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);

    // Vector across y axis of viewport
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Horizontal and delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u.div(image_width as f64);
    let pixel_delta_v = viewport_v.div(image_height as f64);

    let viewport_upper_left = camera_center.clone()
        - Point3::new(0.0, 0.0, focal_length)
        - viewport_u.div(2.0).into()
        - viewport_v.div(2.0).into();
    let pixel00_loc =
        viewport_upper_left.add(0.5) * (pixel_delta_u.clone() + pixel_delta_v.clone()).into();

    // ------------Render------------

    println!("P3"); // P3 means colors are ascii
    println!("{image_width} {image_height}");
    println!("255"); // max color
    for j in 0..image_height {
        //trace!("Scanlines remaining {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone()
                + ((pixel_delta_u.mul(i as f64)) + (pixel_delta_v.mul(j as f64))).into();
            let ray_direction = pixel_center - camera_center.clone();
            let ray = Ray::new(camera_center.clone(), ray_direction.into());
            let pixel_color = ray.ray_color();
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
