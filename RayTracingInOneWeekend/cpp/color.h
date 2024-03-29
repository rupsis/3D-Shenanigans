#ifndef COLOR_H
#define COLOR_H

#include "vec3.h"

#include <iostream>

// Writing out a vec3 "color" alias, aka pixel
void write_color(std::ostream &out, color pixel_color, int samples_per_pixel) {
    auto r = pixel_color.r();
    auto g = pixel_color.g();
    auto b = pixel_color.b();

    // Divide the color by the number of samples and gama-correct for gamma=2.0.
    auto scale = 1.0 / samples_per_pixel;
    r = sqrt(scale * r);
    g = sqrt(scale * g);
    b = sqrt(scale * b);

    // Update each pixel with intensity based on samples. But clamp at
    //  a range of 0 - 0.999
    out << static_cast<int>(255.999 * clamp(r, 0.0, 0.999)) << ' '
        << static_cast<int>(255.999 * clamp(g, 0.0, 0.999)) << ' '
        << static_cast<int>(255.999 * clamp(b, 0.0, 0.999)) << '\n';
}

#endif