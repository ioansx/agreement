use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[wasm_bindgen]
impl CreateUser {
    #[wasm_bindgen(constructor)]
    pub fn new(username: String) -> Self {
        CreateUser { username }
    }
}
