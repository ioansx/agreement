use std::sync::Arc;

use agreement_business::{services::Services, validators::ValidatorsStage1};

pub type ArcState = Arc<AState>;

pub struct AState {
    pub services: Services,
    pub validators: ValidatorsStage1,
}

impl AState {
    pub fn new() -> Self {
        Self {
            services: Services::new(),
            validators: ValidatorsStage1::new(),
        }
    }
}
