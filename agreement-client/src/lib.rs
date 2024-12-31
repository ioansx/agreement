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

    pub async fn root(&self) -> Result<JsValue, JsValue> {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::SameOrigin);

        let request = Request::new_with_str_and_init(&self.addr, &opts)?;

        request.headers().set("Accept", "application/json")?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        // TODO: throw error instead of asserting
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;
        Ok(json)
    }
}
