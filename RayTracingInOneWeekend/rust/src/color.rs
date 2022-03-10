use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        pixel_color.r() * 255.999,
        pixel_color.g() * 255.999,
        pixel_color.b() * 255.999
    );
}
