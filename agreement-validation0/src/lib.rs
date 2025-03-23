mod man_validation0;

pub use man_validation0::ManValidation0;

#[derive(Clone, Debug)]
pub struct Validation0 {
    pub man: ManValidation0,
}

impl Validation0 {
    pub fn new() -> Self {
        Self {
            man: ManValidation0,
        }
    }
}
