use std::{backtrace::Backtrace, fmt::Display};

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

use crate::outdto::ErOutdto;

pub type Eresult<T> = Result<T, Er>;

#[derive(Debug)]
pub struct Er {
    pub knd: ErKind,
    pub src: Option<Box<dyn std::error::Error>>,
    pub bkt: Backtrace,
}

impl Display for Er {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.knd)
    }
}

impl std::error::Error for Er {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.src.as_ref().map(|v| &**v)
    }
}

impl IntoResponse for Er {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self.knd {
            ErKind::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErOutdto {
            id: agreement_common::agreement_id(),
            msg: self.knd.to_string(),
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
        $crate::error::Er {
            knd: $knd,
            src: None,
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
    ( $src:expr, $knd:expr ) => {{
        $crate::error::Er {
            knd: $knd,
            src: Some(Box::new($src)),
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
}

#[derive(Debug)]
pub enum ErKind {
    Internal(String),
}

impl ErKind {
    pub fn internal(msg: impl Into<String>) -> ErKind {
        ErKind::Internal(msg.into())
    }
}

impl Display for ErKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErKind::Internal(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}
