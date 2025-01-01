mod thing_create;

pub use thing_create::CreateThingOutdto;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErOutdto {
    pub id: String,
    pub msg: String,
}
