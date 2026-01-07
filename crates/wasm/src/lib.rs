#![allow(clippy::vec_init_then_push)]
use std::{any::Any, cell::RefCell, fmt::Debug, sync::Arc};

use caustic_core::{
    Color as CoreColor, Image, RenderContext, SceneData, image::ImageError, random_new,
};
use caustic_openscad::{
    resource_resolver::{CodeResource, ResourceResolver},
    run_openscad,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

thread_local! {
static LOADED_SCENE_DATA: RefCell<Option<SceneData>> = const { RefCell::new(None) };
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_RESOURCE_RESOLVER_INTERFACE: &'static str = r#"
export interface WasmResourceResolver {
    get_main(): WasmCodeResource;
}
"#;

#[wasm_bindgen]
extern "C" {
    pub type WasmResourceResolver;

    #[wasm_bindgen(method)]
    pub fn get_main(this: &WasmResourceResolver) -> WasmCodeResource;
}

struct WasmResourceResolverAdapter {
    wasm_resource_resolver: WasmResourceResolver,
}

impl ResourceResolver for WasmResourceResolverAdapter {
    fn get_main(&self) -> Arc<dyn CodeResource> {
        Arc::new(WasmCodeResourceAdapter::new(
            self.wasm_resource_resolver.get_main(),
        ))
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_CODE_RESOURCE_INTERFACE: &'static str = r#"
export interface WasmCodeResource {
    get_code(): string;
}
"#;

#[wasm_bindgen]
extern "C" {
    pub type WasmCodeResource;

    #[wasm_bindgen(method)]
    pub fn get_code(this: &WasmCodeResource) -> String;
}

struct WasmCodeResourceAdapter {
    code: String,
}

impl WasmCodeResourceAdapter {
    pub fn new(wasm_code_resource: WasmCodeResource) -> Self {
        Self {
            code: wasm_code_resource.get_code(),
        }
    }
}

impl CodeResource for WasmCodeResourceAdapter {
    fn get_code(&self) -> &str {
        &self.code
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError> {
        todo!("get_image {filename}")
    }
}

impl Debug for WasmCodeResourceAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmCodeResourceAdapter").finish()
    }
}

#[wasm_bindgen]
pub fn load_openscad(wasm_resource_resolver: WasmResourceResolver) -> Result<LoadResults, JsValue> {
    let resource_resolver = WasmResourceResolverAdapter {
        wasm_resource_resolver,
    };
    let results =
        run_openscad(&resource_resolver).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    LOADED_SCENE_DATA.with(|data| *data.borrow_mut() = Some(results.scene_data));
    Ok(LoadResults {
        output: results.output,
    })
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
pub fn render(xmin: u32, xmax: u32, ymin: u32, ymax: u32) -> Result<Vec<Color>, JsValue> {
    LOADED_SCENE_DATA.with(|data| {
        if let Some(scene_data) = data.borrow().as_ref() {
            let ctx = Arc::new(RenderContext {
                random: random_new(),
            });
            let mut results: Vec<Color> = vec![];

            for y in ymin..ymax {
                for x in xmin..xmax {
                    let pixel_color = scene_data.camera.render(
                        &ctx,
                        x,
                        y,
                        &*scene_data.world,
                        scene_data.lights.clone(),
                    );
                    let color = Color::from(pixel_color);
                    results.push(color);
                }
            }

            Ok(results)
        } else {
            Err(JsValue::from_str("Scene data not loaded"))
        }
    })
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LoadResults {
    pub output: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CameraInfo {
    pub width: u32,
    pub height: u32,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from(color: CoreColor) -> Self {
        Color {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        }
    }
}
