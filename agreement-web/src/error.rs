use std::fmt::Display;

use agreement_common::error::{Err, ErrWrap};
use agreement_models::outdto::ErrorOutdto;
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use tracing::{error, warn};

pub type AerrResult<T> = Result<T, Aerr>;

#[derive(Debug)]
pub struct Aerr(pub ErrWrap);

impl Display for Aerr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Aerr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.src.as_deref()
    }
}

impl IntoResponse for Aerr {
    fn into_response(self) -> axum::response::Response {
        let Self(err) = self;

        let status_code = match err.knd {
            Err::NotFound(_) => StatusCode::NOT_FOUND,
            Err::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErrorOutdto {
            id: agreement_common::agreement_id(),
            msg: err.knd.to_string(),
        };
        let body_ser = serde_json::to_vec(&body).expect("body can always be serialized");

        if let Err::Internal(_) = err.knd {
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
