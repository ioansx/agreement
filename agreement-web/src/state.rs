use std::sync::Arc;

use agreement_services::Services;

pub type ArcState = Arc<AState>;

pub struct AState {
    pub services: Services,
}

impl AState {
    pub fn new() -> Self {
        Self {
            services: Services::new(),
        }
    }
}
