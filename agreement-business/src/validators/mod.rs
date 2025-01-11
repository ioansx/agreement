mod man_validator_stage_1;

pub use man_validator_stage_1::ManValidatorStage1;

pub struct ValidatorsStage1 {
    pub man_stage_1: ManValidatorStage1,
}

impl ValidatorsStage1 {
    pub fn new() -> Self {
        Self {
            man_stage_1: ManValidatorStage1::new(),
        }
    }
}
