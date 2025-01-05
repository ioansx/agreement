use std::{error::Error, fmt::Display};

use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone, Debug)]
pub struct UnladeError {
    pub msg: String,
}

impl Display for UnladeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for UnladeError {}

pub fn map_js_value_to_error_type(value: JsValue) -> UnladeError {
    let msg: String = if let Some(x) = value.dyn_ref::<js_sys::Object>() {
        const MESSAGE: &str = "message";
        let mut message = None;
        for arr in js_sys::Object::entries(&x) {
            let arr = arr
                .dyn_ref::<js_sys::Array>()
                .expect("Each entry in Object::entries is an array.");

            if arr
                .get(0)
                .as_string()
                .expect("The first value in an entry is always a string.")
                == MESSAGE
            {
                message = Some(arr.get(1).as_string().unwrap());
            }
        }

        if let Some(m) = message {
            m
        } else {
            js_sys::JSON::stringify(&value)
                .expect("A JsValue is always convertible into a JSON string.")
                .into()
        }
    } else if let Some(x) = value.dyn_ref::<js_sys::JsString>() {
        x.into()
    } else {
        js_sys::JSON::stringify(&value)
            .expect("A JsValue is always convertible into a JSON string.")
            .into()
    };

    UnladeError { msg }
}
