use std::sync::Arc;

use rust_raytracer_core::{
    Camera, Color, Node, RenderContext, Vector3,
    camera::CameraBuilder,
    image::HtmlImage,
    material::{Lambertian, Metal},
    object::{Group, Sphere},
    random_new,
    texture::ImageTexture,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(
    scene: &str,
    aspect_ratio: f64,
    image_width: u32,
    x: u32,
    y: u32,
) -> Result<JsValue, JsValue> {
    let ctx = RenderContext {
        random: random_new(),
    };

    let (camera, group) = if scene == "threeBall" {
        create_three_ball_scene(aspect_ratio, image_width)
    } else if scene == "earth" {
        create_earth_scene(aspect_ratio, image_width)?
    } else {
        panic!();
    };

    let pixel_color = camera.render(&ctx, x, y, &*group);
    let color = WasmColor::from(pixel_color);

    serde_wasm_bindgen::to_value(&color).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

pub fn create_earth_scene(
    aspect_ratio: f64,
    image_width: u32,
) -> Result<(Arc<Camera>, Arc<dyn Node>), JsValue> {
    let image = HtmlImage::load_url("assets/earth-map.jpg")
        .map_err(|e| JsValue::from_str(&format!("image load error: {:?}", e)))?;
    let earth_texture = Arc::new(ImageTexture::new(image));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = aspect_ratio;
    camera_builder.image_width = image_width;
    camera_builder.samples_per_pixel = 10;
    camera_builder.max_depth = 50;
    camera_builder.vertical_fov = 20.0;
    camera_builder.look_from = Vector3::new(0.0, 0.0, 12.0);
    camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    camera_builder.background = Color::new(0.7, 0.8, 1.0);
    let camera = Arc::new(camera_builder.build());

    Ok((camera, globe))
}

fn create_three_ball_scene(aspect_ratio: f64, image_width: u32) -> (Arc<Camera>, Arc<dyn Node>) {
    let material_ground = Arc::new(Lambertian::new_from_color(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new_from_color(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut group = Group::new();
    group.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    group.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    group.push(Arc::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    group.push(Arc::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = aspect_ratio;
    camera_builder.image_width = image_width;
    camera_builder.background = Color::new(0.7, 0.8, 1.0);
    let camera = Arc::new(camera_builder.build());

    (camera, Arc::new(group))
}

#[derive(Serialize)]
#[wasm_bindgen]
pub struct WasmColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl WasmColor {
    pub fn from(color: Color) -> Self {
        WasmColor {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        }
    }
}
