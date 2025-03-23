use agreement_error::{Errx, Resultx};
use agreement_models::{custom::DateTimeUtc, outdto::ManGetOutdto};

pub struct ManService;

impl ManService {
    pub async fn generate_man_page(&self, command: String) -> Resultx<ManGetOutdto> {
        let pager_option = "-P";
        let pager_as_cat = "cat";
        let output = tokio::process::Command::new("man")
            .args([pager_option, pager_as_cat, &command])
            .env("MANWIDTH", "80")
            .output()
            .await
            .map_err(|e| Errx::einternal(e, "unable to generate man page"))?;

        if !output.status.success() {
            let src = Errx::internal(String::from_utf8_lossy(&output.stderr));
            return Err(Errx::einternal(src, "unable to generate man page"));
        }

        let stdout = String::from_utf8(output.stdout)
            .map_err(|e| Errx::einternal(e, "invalid man page stdout output"))?;

        Ok(ManGetOutdto {
            generated_at: DateTimeUtc::now(),
            output: stdout,
        })
    }
}
