mod vec3;
use std::io::{self, Write};

fn main() {
    let c = vec3::Color::new(0.0, 0.0, 0.0);

    println!("{}", c.x());

    // Image
    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{w} {h}\n255", w = image_width, h = image_height);

    for j in (0..(image_height)).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();

        for i in 0..(image_width) {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\nDone.\n");
}
