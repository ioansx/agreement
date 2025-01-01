use agreement_models::{
    indto::CreateThingIndto,
    outdto::{CreateThingOutdto, ErrorOutdto},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen(js_name = AgreementClient)]
#[derive(Clone, Debug)]
pub struct AgreementClient {
    addr: String,
}

#[wasm_bindgen(js_class = AgreementClient)]
impl AgreementClient {
    #[wasm_bindgen(constructor)]
    pub fn new(addr: String) -> AgreementClient {
        AgreementClient { addr }
    }

    pub async fn create_thing(
        &self,
        indto: CreateThingIndto,
    ) -> Result<CreateThingOutdto, ErrorOutdto> {
        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::SameOrigin);

        let body = serde_json::to_string(&indto).map_err(|e| ErrorOutdto {
            id: "client_error".to_string(),
            msg: e.to_string(),
        })?;
        opts.set_body(&body.into());

        let addr = format!("{}/things", self.addr);
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

        let json_promise = resp.json().map_err(map_js_value_to_error_type)?;
        let json = JsFuture::from(json_promise)
            .await
            .map_err(map_js_value_to_error_type)?;

        let outdto = serde_wasm_bindgen::from_value(json).map_err(|e| ErrorOutdto {
            id: "client_error".to_string(),
            msg: e.to_string(),
        })?;

        Ok(outdto)
    }
}

fn map_js_value_to_error_type(value: JsValue) -> ErrorOutdto {
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

    ErrorOutdto {
        id: "client_error".to_string(),
        msg,
    }
}
