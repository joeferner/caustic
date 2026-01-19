use caustic_openscad::language_server::LanguageServerBackend;
use futures::StreamExt;
use tokio::sync::Mutex;
use tower;
use tower_lsp::LspService;
use tower_lsp::jsonrpc::Request;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLspServer {
    service: Mutex<LspService<LanguageServerBackend>>,
}

#[wasm_bindgen]
impl WasmLspServer {
    #[wasm_bindgen(constructor)]
    pub fn new(output_callback: js_sys::Function) -> Self {
        let (service, mut messages) = LspService::new(LanguageServerBackend::new);

        // Handle Outgoing (Rust -> JS)
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(msg) = messages.next().await {
                let json_str = serde_json::to_string(&msg).unwrap();
                let _ = output_callback.call1(&JsValue::NULL, &JsValue::from_str(&json_str));
            }
        });

        Self {
            service: Mutex::new(service),
        }
    }

    pub async fn notify_client_message(&self, msg: String) {
        let mut service = self.service.lock().await;
        if let Ok(req) = serde_json::from_str::<Request>(&msg) {
            let _ = tower::Service::<Request>::call(&mut *service, req).await;
        }
    }
}
