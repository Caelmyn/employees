use std::error;
use std::fmt::{Debug, Display, Formatter, Result};

/* ---------- */

pub enum Error {
    Context(Box<dyn Display + Send + Sync>),
    Thread(std::io::Error),
    Other(Box<dyn error::Error + Send + Sync>),
}

impl Error {
    pub fn context<D>(reason: D) -> Self
    where
        D: Display + Send + Sync + 'static,
    {
        Self::Context(Box::new(reason))
    }

    pub fn thread(err: std::io::Error) -> Self {
        Self::Thread(err)
    }

    pub fn other<E>(err: E) -> Self
    where
        E: error::Error + Send + Sync + 'static,
    {
        Self::Other(Box::new(err))
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::Context(inner) => f.debug_tuple("Context").field(&format!("{inner}")).finish(),
            Error::Thread(inner) => f.debug_tuple("Thread").field(inner).finish(),
            Error::Other(inner) => f.debug_tuple("Other").field(inner).finish(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Context(arg_name) => {
                write!(f, "Missing argument {arg_name} in context")
            }
            Self::Thread(err) => write!(f, "Failed to run worker thread: {err}"),
            Self::Other(err) => write!(f, "Failed launch worker: {err}"),
        }
    }
}

impl error::Error for Error {}

/* ---------- */
