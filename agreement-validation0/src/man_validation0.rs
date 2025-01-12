use agreement_common::error::ErrResult;

#[derive(Clone, Debug)]
pub struct ManValidation0 {}

impl ManValidation0 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate_man_get(&self, _command: &str) -> ErrResult<()> {
        todo!()
    }
}
