use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn web_ready() -> String {
    "web wasm ready".to_string()
}
