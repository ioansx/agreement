use std::sync::Arc;

use agreement_business::{services::Services, validators::Validators};

pub type ArcState = Arc<AState>;

pub struct AState {
    pub services: Services,
    pub validators: Validators,
}

impl AState {
    pub fn new() -> Self {
        Self {
            services: Services::new(),
            validators: Validators::new(),
        }
    }
}
