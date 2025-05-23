use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::custom::DateTimeUtc;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManGetOutdto {
    pub generated_at: DateTimeUtc,
    pub output: String,
}
