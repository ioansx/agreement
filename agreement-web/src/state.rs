use agreement_services::Services;

pub struct ArState {
    pub services: Services,
}

impl ArState {
    pub fn new() -> Self {
        Self {
            services: Services::new(),
        }
    }
}
