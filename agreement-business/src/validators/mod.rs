mod man_validator;

pub use man_validator::ManValidator;

pub struct Validators {
    pub man: ManValidator,
}

impl Validators {
    pub fn new() -> Self {
        Self {
            man: ManValidator::new(),
        }
    }
}
