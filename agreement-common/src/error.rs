use std::{backtrace::Backtrace, fmt::Display};

#[derive(Debug)]
pub struct ErWrap {
    pub knd: Er,
    pub src: Option<Box<dyn std::error::Error>>,
    pub bkt: Backtrace,
}

impl ErWrap {
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

impl Display for ErWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.knd)
    }
}

impl std::error::Error for ErWrap {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.src.as_ref().map(|v| &**v)
    }
}

#[macro_export]
macro_rules! newer {
    ( $knd:expr ) => {{
        $crate::error::ErWrap {
            knd: $knd,
            src: None,
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
    ( $src:expr, $knd:expr ) => {{
        $crate::error::ErWrap {
            knd: $knd,
            src: Some(Box::new($src)),
            bkt: std::backtrace::Backtrace::force_capture(),
        }
    }};
}

#[derive(Debug)]
pub enum Er {
    NotFound(String),
    Internal(String),
}

impl Er {
    pub fn not_found(msg: impl Into<String>) -> Er {
        Er::NotFound(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Er {
        Er::Internal(msg.into())
    }
}

impl Display for Er {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Er::NotFound(msg) => {
                write!(f, "Not Found: {}", msg)
            }
            Er::Internal(msg) => {
                write!(f, "Internal Server Error: {}", msg)
            }
        }
    }
}
