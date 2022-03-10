mod color;
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
            let pixel_color = vec3::Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            color::write_color(pixel_color)
        }
    }
    eprint!("\nDone.\n");
}
