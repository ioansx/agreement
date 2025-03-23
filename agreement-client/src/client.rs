use agreement_models::{
    indto::CreateThingIndto,
    outdto::{CreateThingOutdto, ErrorOutdto, ManGetOutdto},
};
use agreement_validation0::Validation0;
use unlade::{HttpUnlader, Unlader};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct AgreementClient {
    pub version: String,
    unlader: Unlader,
    validation0: Validation0,
}

#[wasm_bindgen]
impl AgreementClient {
    #[wasm_bindgen(constructor)]
    pub fn new(addr: String) -> AgreementClient {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            unlader: Unlader::new(addr),
            validation0: Validation0::new(),
        }
    }

    pub async fn get_man_page(&self, page: String) -> Result<ManGetOutdto, ErrorOutdto> {
        self.validation0.man.validate_man_get(&page)?;

        let value = self
            .unlader
            .json_get("api/v1/man", &[("page", &page)])
            .await?;

        let outdto: ManGetOutdto = self.unlader.from_js_value(value)?;
        Ok(outdto)
    }

    pub async fn create_thing(
        &self,
        indto: CreateThingIndto,
    ) -> Result<CreateThingOutdto, ErrorOutdto> {
        let outdto = self.unlader.post("/api/v1/things", indto).await?;
        Ok(outdto)
    }
}
