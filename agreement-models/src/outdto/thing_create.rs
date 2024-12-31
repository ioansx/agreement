use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateThingOutdto {
    pub id: String,
    pub name: String,
    pub opt_prop: Option<u64>,
}

#[wasm_bindgen]
impl CreateThingOutdto {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, name: String, opt_prop: Option<u64>) -> Self {
        CreateThingOutdto { id, name, opt_prop }
    }
}
