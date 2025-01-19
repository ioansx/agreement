use agreement_common::{
    error::{Err, ErrResult},
    newer,
};
use agreement_models::{custom::DateTimeUtc, outdto::ManGetOutdto};

pub struct ManService {}

impl ManService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_man_page(&self, command: String) -> ErrResult<ManGetOutdto> {
        let pager_option = "-P";
        let pager_as_cat = "cat";
        let output = tokio::process::Command::new("man")
            .args([pager_option, pager_as_cat, &command])
            .output()
            .await
            .map_err(|e| newer!(e, Err::internal("unable to generate man page")))?;

        if !output.status.success() {
            let source = newer!(Err::internal(String::from_utf8_lossy(&output.stderr)));
            return Err(newer!(source, Err::internal("unable to generate man page")));
        }

        let output_str = String::from_utf8(output.stdout)
            .map_err(|e| newer!(e, Err::internal("invalid man page stdout output")))?;

        Ok(ManGetOutdto {
            generated_at: DateTimeUtc::now(),
            output: output_str,
        })
    }
}
