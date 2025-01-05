use agreement_models::{
    indto::CreateThingIndto,
    outdto::{CreateThingOutdto, ErrorOutdto},
};
use unladeapi::{HttpUnlader, Unlader};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AgreementClient {
    ac: Unlader,
}

#[wasm_bindgen]
impl AgreementClient {
    #[wasm_bindgen(constructor)]
    pub fn new(addr: String) -> AgreementClient {
        Self {
            ac: Unlader::new(addr),
        }
    }

    pub async fn create_thing(
        &self,
        indto: CreateThingIndto,
    ) -> Result<CreateThingOutdto, ErrorOutdto> {
        self.ac.post("things", indto).await?
    }
}
