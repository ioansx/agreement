use agreement_error::Errx;
use serde::{Deserialize, Serialize};
use unlade::UnladeError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorOutdto {
    pub msg: String,
    pub ts: u64,
}

impl From<Errx> for ErrorOutdto {
    fn from(value: Errx) -> Self {
        Self {
            msg: value.knd.to_string(),
            ts: value.ts,
        }
    }
}

impl From<UnladeError> for ErrorOutdto {
    fn from(value: UnladeError) -> Self {
        Self {
            msg: value.msg,
            ts: value.ts,
        }
    }
}
