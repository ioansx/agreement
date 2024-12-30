use std::{backtrace::Backtrace, fmt::Display};

#[derive(Debug)]
pub struct Error {
    pub knd: ErrKind,
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
macro_rules! new_err {
    ( $knd:expr ) => {{
        $crate::error::Error {
            knd: $knd,
            src: None,
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
    ( $knd:expr, $src:expr ) => {{
        $crate::error::Error {
            knd: $knd,
            src: Some(Box::new($src)),
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
}

#[derive(Debug)]
pub enum ErrKind {
    Bare(String),
}

impl ErrKind {
    pub fn bare(msg: impl Into<String>) -> ErrKind {
        ErrKind::Bare(msg.into())
    }
}

impl Display for ErrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrKind::Bare(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}
