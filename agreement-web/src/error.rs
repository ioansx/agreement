use agreement_error::{Errx, Kindx};
use agreement_models::outdto::ErrorOutdto;

pub type WebResultx<T> = Result<T, WebErrx>;

#[derive(Debug)]
pub struct WebErrx(pub Errx);

impl From<Errx> for WebErrx {
    fn from(value: Errx) -> Self {
        Self(value)
    }
}

impl axum::response::IntoResponse for WebErrx {
    fn into_response(self) -> axum::response::Response {
        let Self(errx) = self;
        let chain = errx.chain();
        let Errx { bkt, knd, ts, .. } = errx;

        let status_code = match knd {
            Kindx::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            Kindx::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Kindx::Validation(_) => axum::http::StatusCode::BAD_REQUEST,
        };

        let body = ErrorOutdto {
            msg: knd.to_string(),
            ts,
        };
        let body_ser = serde_json::to_vec(&body).expect("error body should be serializable");

        let bkt = bkt
            .as_ref()
            .map(|x| x.to_string())
            .unwrap_or("".to_string());
        if let Kindx::Internal(_) = knd {
            tracing::error!(msg = %body.msg, ts = %ts, chain = ?chain, bkt = %bkt);
        } else {
            tracing::warn!(msg = %body.msg, ts = %ts, chain = ?chain, bkt = %bkt);
        };

        axum::http::Response::builder()
            .status(status_code)
            .header(
                axum::http::header::CONTENT_TYPE,
                axum::http::header::HeaderValue::from_static("application/json"),
            )
            .body(body_ser.into())
            .expect("error response should always be built")
    }
}
