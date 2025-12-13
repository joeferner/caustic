#![allow(clippy::vec_init_then_push)]
use std::{cell::RefCell, sync::Arc};

use rust_raytracer_core::{Color, RenderContext, SceneData, random_new};
use rust_raytracer_openscad::openscad_string_to_scene_data;
use serde::Serialize;
use wasm_bindgen::prelude::*;

thread_local! {
static LOADED_SCENE_DATA: RefCell<Option<SceneData>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn load_openscad(input: &str) -> Result<(), JsValue> {
    let scene_data =
        openscad_string_to_scene_data(input).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    LOADED_SCENE_DATA.with(|data| *data.borrow_mut() = Some(scene_data));
    Ok(())
}

#[wasm_bindgen]
pub fn get_camera_info() -> Result<CameraInfo, JsValue> {
    LOADED_SCENE_DATA.with(|data| {
        if let Some(scene_data) = data.borrow().as_ref() {
            let width = scene_data.camera.image_width();
            let height = scene_data.camera.image_height();
            Ok(CameraInfo { width, height })
        } else {
            Err(JsValue::from_str("Scene data not loaded"))
        }
    })
}

#[wasm_bindgen]
pub fn render(xmin: u32, xmax: u32, ymin: u32, ymax: u32) -> Result<Vec<WasmColor>, JsValue> {
    LOADED_SCENE_DATA.with(|data| {
        if let Some(scene_data) = data.borrow().as_ref() {
            let ctx = Arc::new(RenderContext {
                random: random_new(),
            });
            let mut results: Vec<WasmColor> = vec![];

            for y in ymin..ymax {
                for x in xmin..xmax {
                    let pixel_color = scene_data.camera.render(
                        &ctx,
                        x,
                        y,
                        &*scene_data.world,
                        scene_data.lights.clone(),
                    );
                    let color = WasmColor::from(pixel_color);
                    results.push(color);
                }
            }

            Ok(results)
        } else {
            Err(JsValue::from_str("Scene data not loaded"))
        }
    })
}

#[derive(Serialize)]
#[wasm_bindgen]
pub struct CameraInfo {
    pub width: u32,
    pub height: u32,
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
