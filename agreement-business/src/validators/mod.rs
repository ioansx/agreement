mod man_validator_stage_1;
mod man_validator_stage_2;

pub use man_validator_stage_1::ManValidatorStage1;
pub use man_validator_stage_2::ManValidatorStage2;

pub struct ValidatorsStage1 {
    pub man: ManValidatorStage1,
}

impl ValidatorsStage1 {
    pub fn new() -> Self {
        Self {
            man: ManValidatorStage1::new(),
        }
    }
}

pub struct ValidatorsStage2 {
    pub man: ManValidatorStage2,
}

impl ValidatorsStage2 {
    pub fn new() -> Self {
        Self {
            man: ManValidatorStage2::new(),
        }
    }
}
