use agreement_common::error::ErrResult;

pub struct ManValidatorStage1 {}

impl ManValidatorStage1 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn sanity_check_man_get(&self, _command: &str) -> ErrResult<()> {
        todo!()
    }
}
