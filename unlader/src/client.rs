use std::future::Future;

use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response, UrlSearchParams};

use crate::error::{map_js_value_to_error_type, UnladeError};

const HEADERS_FOR_JSON_REQUEST: &[(&str, &str)] = &[
    ("Content-Type", "application/json"),
    ("Accept", "application/json"),
];

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

    // pub fn to_js_value<T: Serialize>(&self, value: T) -> Result<JsValue, UnladeError> {
    //     let serde_json::to_string(&value)
    //         .map_err(|e| UnladeError { msg: e.to_string() })?
    // }

    pub fn from_js_value<T: DeserializeOwned>(&self, value: JsValue) -> Result<T, UnladeError> {
        serde_wasm_bindgen::from_value(value).map_err(|e| UnladeError { msg: e.to_string() })
    }

    pub async fn json_get(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<JsValue, UnladeError> {
        let mut resource = if path.is_empty() {
            self.addr.clone()
        } else {
            format!("{}/{}", self.addr, path)
        };

        if !query.is_empty() {
            let query_params = UrlSearchParams::new().map_err(map_js_value_to_error_type)?;
            for (key, value) in query.iter() {
                query_params.append(key, value);
            }

            resource = format!("{}?{}", resource, query_params.to_string());
        }

        let headers = Headers::new().map_err(map_js_value_to_error_type)?;
        for (key, value) in HEADERS_FOR_JSON_REQUEST {
            headers
                .set(key, value)
                .map_err(map_js_value_to_error_type)?;
        }

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_headers(&headers);

        let request =
            Request::new_with_str_and_init(&resource, &opts).map_err(map_js_value_to_error_type)?;

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

        Ok(json)
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
