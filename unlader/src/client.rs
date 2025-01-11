use std::future::Future;

use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::error::{map_js_value_to_error_type, UnladeError};

#[derive(Clone, Debug)]
pub struct Unlader {
    addr: String,
}

pub trait HttpUnlader {
    fn post<I: Serialize, O: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        input: I,
    ) -> impl Future<Output = Result<O, UnladeError>>;
}

impl Unlader {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub async fn get(&self, path: &str) -> Result<JsValue, UnladeError> {
        todo!()
    }
}

impl HttpUnlader for Unlader {
    fn post<I: Serialize, O: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        input: I,
    ) -> impl Future<Output = Result<O, UnladeError>> {
        // TODO: This is dirty, clean it up.
        let body = serde_json::to_string(&input)
            .map_err(|e| UnladeError { msg: e.to_string() })
            .unwrap();

        async move {
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::SameOrigin);

            opts.set_body(&body.into());

            let addr = format!("{}/{}", self.addr, path);
            let request =
                Request::new_with_str_and_init(&addr, &opts).map_err(map_js_value_to_error_type)?;

            request
                .headers()
                .set("Content-Type", "application/json")
                .map_err(map_js_value_to_error_type)?;
            request
                .headers()
                .set("Accept", "application/json")
                .map_err(map_js_value_to_error_type)?;

            let window =
                web_sys::window().expect("Window should be defined, are you running in a browser?");
            let resp_value = JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(map_js_value_to_error_type)?;

            let resp: Response = resp_value.dyn_into().map_err(map_js_value_to_error_type)?;

            // TODO: verify 2xx/3xx response
            let json_promise = resp.json().map_err(map_js_value_to_error_type)?;
            let json = JsFuture::from(json_promise)
                .await
                .map_err(map_js_value_to_error_type)?;

            let outdto = serde_wasm_bindgen::from_value(json)
                .map_err(|e| UnladeError { msg: e.to_string() })?;

            Ok(outdto)
        }
    }
}
