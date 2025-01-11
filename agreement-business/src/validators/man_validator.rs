use agreement_common::error::ErrResult;
use agreement_models::indto::ManGetIndto;

pub struct ManValidator {}

impl ManValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn sanity_check_man_get(&self, indto: &ManGetIndto) -> ErrResult<()> {
        todo!()
    }
}
