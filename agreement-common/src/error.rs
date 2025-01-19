use std::{backtrace::Backtrace, fmt::Display};

pub type ErrResult<T> = Result<T, ErrWrap>;

#[derive(Debug)]
pub struct ErrWrap {
    pub knd: Err,
    pub src: Option<Box<dyn std::error::Error>>,
    pub bkt: Backtrace,
}

impl ErrWrap {
    pub fn chain(&self) -> Vec<String> {
        let mut chain: Vec<String> = Vec::new();
        let mut source = Some(self as &dyn std::error::Error);
        while let Some(err) = source {
            chain.push(err.to_string());
            source = err.source();
        }
        chain
    }
}

impl Display for ErrWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.knd)
    }
}

impl std::error::Error for ErrWrap {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.src.as_deref()
    }
}

#[macro_export]
macro_rules! newer {
    ( $knd:expr ) => {{
        $crate::error::ErrWrap {
            knd: $knd,
            src: None,
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
    ( $src:expr, $knd:expr ) => {{
        $crate::error::ErrWrap {
            knd: $knd,
            src: Some(Box::new($src)),
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
}

#[derive(Debug)]
pub enum Err {
    NotFound(String),
    Internal(String),
}

impl Err {
    pub fn not_found(msg: impl Into<String>) -> Err {
        Err::NotFound(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Err {
        Err::Internal(msg.into())
    }
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::NotFound(msg) => {
                write!(f, "Not Found: {}", msg)
            }
            Err::Internal(msg) => {
                write!(f, "Internal Server Error: {}", msg)
            }
        }
    }
}
