use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateThingIndto {
    pub name: String,
    pub opt_prop: Option<u64>,
}

#[wasm_bindgen]
impl CreateThingIndto {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        CreateThingIndto {
            name,
            opt_prop: None,
        }
    }
}
