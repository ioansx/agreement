use std::{backtrace::Backtrace, fmt::Display};

#[derive(Debug)]
pub struct Error {
    pub knd: ErKind,
    pub src: Option<Box<dyn std::error::Error>>,
    pub bkt: Backtrace,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.knd)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.src.as_ref().map(|v| &**v)
    }
}

#[macro_export]
macro_rules! new_error {
    ( $knd:expr ) => {{
        $crate::error::Error {
            knd: $knd,
            src: None,
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
    ( $src:expr, $knd:expr ) => {{
        $crate::error::Error {
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
