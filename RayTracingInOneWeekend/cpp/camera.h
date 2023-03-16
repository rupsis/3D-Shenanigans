#ifndef CAMERA_H
#define CAMERA_H

#include "util.h"

class camera {
    public: 
        camera() {
            auto aspect_ratio = 16.0 / 9.0;
            auto viewport_height = 2.0;
            auto viewport_width = aspect_ratio * viewport_height;
            auto focal_length = 1.0;

            origin = point3(0,0,0);
            horizontal = vec3(viewport_width, 0.0, 0.0);
            // I'm going to define Z-up, like blender 
            vertical = vec3(0.0, 0.0, viewport_height);
            lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, focal_length, 0);
        }

        ray get_ray(double u, double v) const {
            return ray(origin, lower_left_corner + (u*horizontal) + (v*vertical) - origin);
        }

        // The camera is fixed, so just update the origin to move the scene around
        void set_x_y(double x, double y){
            origin = point3(x,y,0);
        }

    private:
        point3 origin;
        point3 lower_left_corner;
        vec3 horizontal;
        vec3 vertical;
};
#endif