use agreement_models::{
    indto::CreateThingIndto,
    outdto::{CreateThingOutdto, ErrorOutdto, ManGetOutdto},
};
use unlader::{HttpUnlader, Unlader};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AgreementClient {
    pub version: &'static str,
    unlader: Unlader,
}

#[wasm_bindgen]
impl AgreementClient {
    #[wasm_bindgen(constructor)]
    pub fn new(addr: String) -> AgreementClient {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            unlader: Unlader::new(addr),
        }
    }

    pub async fn get_man_page(&self, command: String) -> Result<ManGetOutdto, ErrorOutdto> {
        let value = self
            .unlader
            .json_get("man", &[("command", &command)])
            .await?;

        self.unlader.from_js_value(value)?
    }

    pub async fn create_thing(
        &self,
        indto: CreateThingIndto,
    ) -> Result<CreateThingOutdto, ErrorOutdto> {
        self.unlader.post("things", indto).await?
    }
}
