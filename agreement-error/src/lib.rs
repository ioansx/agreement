use std::{backtrace::Backtrace, error::Error, fmt::Display};

pub type Resultx<T> = Result<T, Errx>;

#[derive(Debug)]
pub struct Errx {
    pub src: Option<Box<dyn Error>>,
    pub bkt: Option<Backtrace>,
    pub knd: Kindx,
    pub ts: u64,
}

impl Errx {
    pub fn new(src: Option<Box<dyn Error>>, knd: Kindx) -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be ordered")
            .as_millis() as u64;
        match src {
            Some(src) => match src.downcast::<Errx>() {
                Ok(mut errx) => {
                    let bkt = errx.bkt.take();
                    Self {
                        src: Some(errx),
                        bkt,
                        knd,
                        ts,
                    }
                }
                Err(err) => Self {
                    src: Some(err),
                    bkt: Some(Backtrace::force_capture()),
                    knd,
                    ts,
                },
            },
            None => Self {
                src: None,
                bkt: Some(Backtrace::force_capture()),
                knd,
                ts,
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

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::new(None, Kindx::Validation(msg.into()))
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

#[derive(Debug)]
pub enum Kindx {
    NotFound(String),
    Internal(String),
    Validation(String),
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
            Kindx::Validation(msg) => {
                write!(f, "Validation Error: {}", msg)
            }
        }
    }
}
