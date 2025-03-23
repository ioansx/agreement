use agreement_error::Resultx;

#[derive(Clone, Debug)]
pub struct ManValidation0;

impl ManValidation0 {
    pub fn validate_man_get(&self, _command: &str) -> Resultx<()> {
        // TODO: validation
        Ok(())
    }
}
