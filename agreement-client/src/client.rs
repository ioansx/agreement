use agreement_models::{
    indto::{CreateThingIndto, ManGetIndto},
    outdto::{CreateThingOutdto, ErrorOutdto, ManGetOutdto},
};
use unlader::{HttpUnlader, Unlader};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct AgreementClient {
    version: String,
    unlader: Unlader,
}

#[wasm_bindgen]
impl AgreementClient {
    #[wasm_bindgen(constructor)]
    pub fn new(addr: String) -> AgreementClient {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            unlader: Unlader::new(addr),
        }
    }

    pub async fn get_man_page(&self, indto: ManGetIndto) -> Result<ManGetOutdto, ErrorOutdto> {
        // self.unlader.get("man", indto).await?
        todo!()
    }

    pub async fn create_thing(
        &self,
        indto: CreateThingIndto,
    ) -> Result<CreateThingOutdto, ErrorOutdto> {
        self.unlader.post("things", indto).await?
    }
}
