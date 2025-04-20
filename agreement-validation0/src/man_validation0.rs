use agreement_error::{Errx, Resultx};

const VALID_PAGES: &[&str] = &["bash", "cd", "grep", "ls", "man", "tar"];

#[derive(Clone, Debug)]
pub struct ManValidation0;

impl ManValidation0 {
    pub fn validate_man_get(&self, page: &str) -> Resultx<()> {
        if page.is_empty() {
            return Err(Errx::validation("page is empty"));
        }

        if !VALID_PAGES.contains(&page) {
            return Err(Errx::validation(format!(
                "page must be one of {VALID_PAGES:?}"
            )));
        }

        Ok(())
    }
}
