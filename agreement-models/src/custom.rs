use chrono::Utc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DateTimeUtc(pub String);

impl DateTimeUtc {
    pub fn now() -> Self {
        Self(Utc::now().to_rfc3339())
    }
}
