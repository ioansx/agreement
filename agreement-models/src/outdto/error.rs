use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorOutdto {
    pub id: String,
    pub msg: String,
}
