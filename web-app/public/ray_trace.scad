module camera(
  aspect_ratio,
  image_width,
  samples_per_pixel,
  max_depth,
  defocus_angle,
  background,
  look_at,
  look_from,
  up
){}

function checker(scale = 1, even = [0, 0, 0], odd = [1, 1, 1]) = 1;
