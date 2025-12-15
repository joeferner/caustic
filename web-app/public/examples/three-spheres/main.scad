$fa = 1;
$fs = 0.4;

include <ray_trace.scad>

// camera
camera(
  aspect_ratio=16.0 / 9.0,
  image_width=600,
  samples_per_pixel=10,
  max_depth=50,
  defocus_angle=0.6,
  focus_distance=1.0,
  background=[0.7, 0.8, 1.0]
);

// ground
lambertian(checker(scale=0.32, even=[0.2, 0.3, 0.1], odd=[0.9, 0.9, 0.9]))
  translate([0.0, -1.0, -100.5])
    sphere(r=100);

// center
translate([0.0, -1.2, 0.0])
  sphere(r=0.5);

// left
translate([-1.0, -1.0, 0.0])
  sphere(r=0.5);
translate([-1.0, -1.0, 0.0])
  sphere(r=0.4);

// right
translate([1.0, -1.0, 0.0])
  sphere(r=0.5);
