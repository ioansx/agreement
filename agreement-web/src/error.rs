use std::fmt::Display;

use agreement_common::error::Er;
use agreement_models::outdto::ErrorOutdto;
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use tracing::{error, warn};

pub type AResult<T> = Result<T, AEr>;

#[derive(Debug)]
pub struct AEr(pub agreement_common::error::ErWrap);

impl Display for AEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.src.as_ref().map(|v| &**v)
    }
}

impl IntoResponse for AEr {
    fn into_response(self) -> axum::response::Response {
        let Self(err) = self;

        let status_code = match err.knd {
            Er::NotFound(_) => StatusCode::NOT_FOUND,
            Er::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErrorOutdto {
            id: agreement_common::agreement_id(),
            msg: err.knd.to_string(),
        };
        let body_ser = serde_json::to_vec(&body).expect("body can always be serialized");

        if let Er::Internal(_) = err.knd {
            error!(id = %body.id, msg = %body.msg, chain = ?err.chain());
        } else {
            warn!(id = %body.id, msg = %body.msg, chain = ?err.chain());
        };

        Response::builder()
            .status(status_code)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            )
            .body(body_ser.into())
            .expect("error response can always be built")
    }
}
