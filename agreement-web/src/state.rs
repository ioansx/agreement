use std::sync::Arc;

use agreement_business::{services::Services, validators::Validators};
use agreement_validation0::Validation0;

pub type ArcState = Arc<AState>;

pub struct AState {
    pub services: Services,
    pub validators: Validators,
    pub validation0: Validation0,
}

impl AState {
    pub fn new() -> Self {
        Self {
            services: Services::new(),
            validators: Validators::new(),
            validation0: Validation0::new(),
        }
    }
}
