#![allow(clippy::vec_init_then_push)]
pub mod checkered_spheres;
pub mod earth;
pub mod random_spheres;
pub mod three_spheres;

use std::sync::Arc;

use rust_raytracer_core::{Camera, RenderContext, object::Node};

use crate::scene::{
    checkered_spheres::create_checkered_spheres_scene, earth::create_earth_scene,
    random_spheres::create_random_spheres_scene, three_spheres::create_three_spheres_scene,
};

pub enum Scene {
    ThreeSpheres,
    RandomSpheres,
    CheckeredSpheres,
    Earth,
}

pub fn get_scene(ctx: &RenderContext, scene: Scene) -> (Arc<Camera>, Arc<dyn Node>) {
    match scene {
        Scene::ThreeSpheres => create_three_spheres_scene(ctx),
        Scene::RandomSpheres => create_random_spheres_scene(ctx),
        Scene::CheckeredSpheres => create_checkered_spheres_scene(ctx),
        Scene::Earth => create_earth_scene(ctx),
    }
}
