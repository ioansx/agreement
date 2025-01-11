use agreement_common::{
    error::{Er, ErResult},
    newer,
};
use agreement_models::{indto::ManGetIndto, outdto::ManGetOutdto};

pub struct ManService {}

impl ManService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_man_page(&self, indto: ManGetIndto) -> ErResult<ManGetOutdto> {
        let force_non_localized_man_pages = "-o";
        let pager_option = "-P";
        let pager_as_cat = "cat";
        let output = tokio::process::Command::new("man")
            .args(&[
                force_non_localized_man_pages,
                pager_option,
                pager_as_cat,
                &indto.command,
            ])
            .output()
            .await
            .map_err(|e| newer!(e, Er::internal("unable to generate man page")))?;

        if !output.status.success() {
            return Err(newer!(Er::internal("unable to generate man page")));
        }

        let output_str = String::from_utf8(output.stdout)
            .map_err(|e| newer!(e, Er::internal("invalid man page output")))?;

        Ok(ManGetOutdto { output: output_str })
    }
}
