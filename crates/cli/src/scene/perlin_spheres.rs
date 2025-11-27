use std::sync::Arc;

use rust_raytracer_core::{
    Camera, Node, RenderContext, Vector3,
    camera::CameraBuilder,
    material::Lambertian,
    object::{BoundingVolumeHierarchy, Sphere},
    texture::NoiseTexture,
};

pub fn create_perlin_spheres_scene(ctx: &RenderContext) -> (Arc<Camera>, Arc<dyn Node>) {
    let texture_perlin = Arc::new(NoiseTexture::new(&*ctx.random));
    let material_perlin = Arc::new(Lambertian::new(texture_perlin));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_perlin.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 2.0, 0.0),
        2.0,
        material_perlin,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 16.0 / 9.0;
    camera_builder.image_width = 400;
    camera_builder.samples_per_pixel = 100;
    camera_builder.max_depth = 50;
    camera_builder.vertical_fov = 20.0;
    camera_builder.look_from = Vector3::new(13.0, 2.0, 3.0);
    camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    let camera = Arc::new(camera_builder.build());

    (camera, world)
}
