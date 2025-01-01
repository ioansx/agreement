use std::fmt::Display;

use agreement_common::error::ErKind;
use agreement_models::outdto::ErrorOutdto;
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

pub type Ersult<T> = Result<T, Er>;

#[derive(Debug)]
pub struct Er(pub agreement_common::error::Error);

impl Display for Er {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Er {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.src.as_ref().map(|v| &**v)
    }
}

impl IntoResponse for Er {
    fn into_response(self) -> axum::response::Response {
        let Self(err) = self;

        let status_code = match err.knd {
            ErKind::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErrorOutdto {
            id: agreement_common::agreement_id(),
            msg: err.knd.to_string(),
        };
        let body_ser = serde_json::to_vec(&body).expect("body can always be serialized");

        // TODO: log error chain

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

#[macro_export]
macro_rules! newer {
    ( $knd:expr ) => {{
        $crate::error::Er(agreement_common::new_error!($knd))
    }};
    ( $src:expr, $knd:expr ) => {{
        $crate::error::Er(agreement_common::new_error!($src, $knd))
    }};
}
