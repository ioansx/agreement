use agreement_models::{
    indto::CreateThingIndto,
    outdto::{CreateThingOutdto, ErrorOutdto},
};
use unlader::{HttpUnlader, Unlader};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AgreementClient {
    unlader: Unlader,
}

#[wasm_bindgen]
impl AgreementClient {
    #[wasm_bindgen(constructor)]
    pub fn new(addr: String) -> AgreementClient {
        Self {
            unlader: Unlader::new(addr),
        }
    }

    pub async fn create_thing(
        &self,
        indto: CreateThingIndto,
    ) -> Result<CreateThingOutdto, ErrorOutdto> {
        self.unlader.post("things", indto).await?
    }
}
