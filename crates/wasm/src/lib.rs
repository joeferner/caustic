#![allow(clippy::vec_init_then_push)]
use std::{any::Any, cell::RefCell, fmt::Debug, sync::Arc};

use caustic_core::{
    Color as CoreColor, Image, RenderContext, SceneData, image::ImageError, random_new,
};
use caustic_openscad::{run_openscad, source::Source};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

thread_local! {
static LOADED_SCENE_DATA: RefCell<Option<SceneData>> = const { RefCell::new(None) };
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_CODE_RESOURCE_INTERFACE: &'static str = r#"
export interface WasmSource {
    get_code(): string;
    get_image(filename: string): WasmImage;
}
"#;

#[wasm_bindgen]
extern "C" {
    pub type WasmSource;

    #[wasm_bindgen(method)]
    pub fn get_code(this: &WasmSource) -> String;

    #[wasm_bindgen(method)]
    pub fn get_image(this: &WasmSource, filename: &str) -> WasmImage;
}

// Add this wrapper struct
struct SendSyncWasmSource(WasmSource);

// SAFETY: In WASM, all JS interactions happen on the main thread.
// The wasm-bindgen runtime ensures thread safety.
unsafe impl Send for SendSyncWasmSource {}
unsafe impl Sync for SendSyncWasmSource {}

impl std::ops::Deref for SendSyncWasmSource {
    type Target = WasmSource;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct WasmSourceAdapter {
    wasm_source: SendSyncWasmSource,
    code: String,
}

impl WasmSourceAdapter {
    pub fn new(wasm_source: WasmSource) -> Self {
        let code = wasm_source.get_code();
        Self {
            wasm_source: SendSyncWasmSource(wasm_source),
            code,
        }
    }
}

impl Source for WasmSourceAdapter {
    fn get_code(&self) -> &str {
        &self.code
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError> {
        let image = self.wasm_source.get_image(filename);
        Ok(Arc::new(WasmImageAdapter::new(image)))
    }
}

impl Debug for WasmSourceAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmCodeResourceAdapter").finish()
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_IMAGE_INTERFACE: &'static str = r#"
export interface WasmImage {
    get_width(): number;
    get_height(): number;
    get_data(): Color[];
}
"#;

#[wasm_bindgen]
extern "C" {
    pub type WasmImage;

    #[wasm_bindgen(method)]
    pub fn get_width(this: &WasmImage) -> u32;

    #[wasm_bindgen(method)]
    pub fn get_height(this: &WasmImage) -> u32;

    #[wasm_bindgen(method)]
    pub fn get_data(this: &WasmImage) -> Vec<Color>;
}

struct WasmImageAdapter {
    width: u32,
    height: u32,
    data: Vec<CoreColor>,
}

impl WasmImageAdapter {
    pub fn new(wasm_image: WasmImage) -> Self {
        Self {
            width: wasm_image.get_width(),
            height: wasm_image.get_height(),
            data: wasm_image.get_data().iter().map(Color::to).collect(),
        }
    }
}

impl Image for WasmImageAdapter {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get_pixel(&self, x: u32, y: u32) -> Option<CoreColor> {
        let index = ((y * self.width) + x) as usize;
        self.data.get(index).copied()
    }
}

impl Debug for WasmImageAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmImageAdapter").finish()
    }
}

#[wasm_bindgen]
pub fn load_openscad(wasm_source: WasmSource) -> Result<LoadResults, JsValue> {
    let source = Arc::new(WasmSourceAdapter::new(wasm_source));
    let results = run_openscad(source).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
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

    pub fn to(&self) -> CoreColor {
        CoreColor::new(
            (self.r as f64) / 255.0,
            (self.g as f64) / 255.0,
            (self.b as f64) / 255.0,
        )
    }
}
