mod man_service;

pub use man_service::ManService;

pub struct Services {
    pub man: ManService,
}

impl Services {
    pub fn new() -> Self {
        Self {
            man: ManService::new(),
        }
    }
}
