use std::{backtrace::Backtrace, error::Error, fmt::Display};

use agreement_models::outdto::ErrorOutdto;

pub type Resultx<T> = Result<T, Errx>;

#[derive(Debug)]
pub struct Errx {
    pub id: String,
    pub src: Option<Box<dyn Error>>,
    pub bkt: Option<Backtrace>,
    pub knd: Kindx,
}

impl Errx {
    pub fn new(src: Option<Box<dyn Error>>, knd: Kindx) -> Self {
        let id = agreement_common::agreement_id();
        match src {
            Some(src) => match src.downcast::<Errx>() {
                Ok(mut errx) => {
                    let bkt = errx.bkt.take();
                    Self {
                        id,
                        src: Some(errx),
                        bkt,
                        knd,
                    }
                }
                Err(err) => Self {
                    id,
                    src: Some(err),
                    bkt: Some(Backtrace::force_capture()),
                    knd,
                },
            },
            None => Self {
                id,
                src: None,
                bkt: Some(Backtrace::force_capture()),
                knd,
            },
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(None, Kindx::Internal(msg.into()))
    }

    pub fn einternal(src: impl Error + 'static, msg: impl Into<String>) -> Self {
        Self::new(Some(Box::new(src)), Kindx::Internal(msg.into()))
    }

    pub fn notfound(msg: impl Into<String>) -> Self {
        Self::new(None, Kindx::NotFound(msg.into()))
    }

    pub fn enotfound(src: impl Error + 'static, msg: impl Into<String>) -> Self {
        Self::new(Some(Box::new(src)), Kindx::NotFound(msg.into()))
    }

    pub fn chain(&self) -> Vec<String> {
        let mut chain: Vec<String> = Vec::new();
        let mut source = Some(self as &dyn Error);
        while let Some(err) = source {
            chain.push(err.to_string());
            source = err.source();
        }
        chain
    }
}

impl Display for Errx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.knd)
    }
}

impl Error for Errx {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.src.as_deref()
    }
}

impl axum::response::IntoResponse for Errx {
    fn into_response(self) -> axum::response::Response {
        let chain = self.chain();
        let Self { id, bkt, knd, .. } = self;

        let status_code = match knd {
            Kindx::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            Kindx::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErrorOutdto {
            id,
            msg: knd.to_string(),
        };
        let body_ser = serde_json::to_vec(&body).expect("error body should be serializable");

        let bkt = bkt
            .as_ref()
            .map(|x| x.to_string())
            .unwrap_or("".to_string());
        if let Kindx::Internal(_) = knd {
            tracing::error!(id = %body.id, msg = %body.msg, chain = ?chain, bkt = %bkt);
        } else {
            tracing::warn!(id = %body.id, msg = %body.msg, chain = ?chain, bkt = %bkt);
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

impl From<Errx> for ErrorOutdto {
    fn from(value: Errx) -> Self {
        ErrorOutdto {
            id: agreement_common::agreement_id(),
            msg: value.knd.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Kindx {
    NotFound(String),
    Internal(String),
}

impl Kindx {
    pub fn not_found(msg: impl Into<String>) -> Kindx {
        Kindx::NotFound(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Kindx {
        Kindx::Internal(msg.into())
    }
}

impl Display for Kindx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kindx::NotFound(msg) => {
                write!(f, "Not Found: {}", msg)
            }
            Kindx::Internal(msg) => {
                write!(f, "Internal Server Error: {}", msg)
            }
        }
    }
}
