use indto::CreateThingIndto;
use outdto::CreateThingOutdto;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub use agreement_models::*;

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
    ) -> Result<CreateThingOutdto, JsValue> {
        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::SameOrigin);
        opts.set_body(&serde_json::to_string(&indto).unwrap().into());

        let addr = format!("{}/things", self.addr);
        let request = Request::new_with_str_and_init(&addr, &opts)?;

        request.headers().set("Content-Type", "application/json")?;
        request.headers().set("Accept", "application/json")?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        // TODO: throw error instead of asserting
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;
        let outdto = serde_wasm_bindgen::from_value(json).unwrap();
        // let value = json.into();
        Ok(outdto)
    }

    // pub async fn root(&self) -> Result<JsValue, JsValue> {
    //     let opts = RequestInit::new();
    //     opts.set_method("GET");
    //     opts.set_mode(RequestMode::SameOrigin);
    //
    //     let request = Request::new_with_str_and_init(&self.addr, &opts)?;
    //
    //     request.headers().set("Accept", "application/json")?;
    //
    //     let window = web_sys::window().unwrap();
    //     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    //
    //     // TODO: throw error instead of asserting
    //     assert!(resp_value.is_instance_of::<Response>());
    //     let resp: Response = resp_value.dyn_into().unwrap();
    //
    //     let json = JsFuture::from(resp.json()?).await?;
    //     Ok(json)
    // }
}
