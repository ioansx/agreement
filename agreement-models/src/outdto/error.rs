use agreement_common::agreement_id;
use serde::{Deserialize, Serialize};
use unladeapi::UnladeError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorOutdto {
    pub id: String,
    pub msg: String,
}

impl From<UnladeError> for ErrorOutdto {
    fn from(value: UnladeError) -> Self {
        Self {
            id: agreement_id(),
            msg: value.msg,
        }
    }
}
