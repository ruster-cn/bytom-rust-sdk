use crate::client::response::Response;
use crate::error::ErrorKind::{SerdeJsonError, Utf8Error};
use serde::export::Formatter;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::*;

#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    HttpError(http::Error),
    HyperError(hyper::Error),
    BtmError(BtmError),
    SerdeJsonError(serde_json::error::Error),
    Utf8Error(std::str::Utf8Error),
}

impl Error {
    /// Return true if the underlying error has the same type as T.
    pub fn is<T: std::error::Error + 'static>(&self) -> bool {
        self.get_ref().is::<T>()
    }

    /// Return a reference to the lower level, inner error.
    pub fn get_ref(&self) -> &(dyn std::error::Error + 'static) {
        use self::ErrorKind::*;

        match self.inner {
            HttpError(ref e) => e,
            HyperError(ref e) => e,
            BtmError(ref e) => e,
            SerdeJsonError(ref e) => e,
            Utf8Error(ref e) => e,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.get_ref().source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.get_ref(), f)
    }
}

impl From<http::Error> for Error {
    fn from(s: http::Error) -> Self {
        Error {
            inner: ErrorKind::HttpError(s),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(s: hyper::Error) -> Self {
        Error {
            inner: ErrorKind::HyperError(s),
        }
    }
}

impl From<BtmError> for Error {
    fn from(s: BtmError) -> Self {
        use crate::error::ErrorKind::BtmError;
        Error { inner: BtmError(s) }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(s: serde_json::error::Error) -> Self {
        Error {
            inner: SerdeJsonError(s),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(s: std::str::Utf8Error) -> Self {
        Error {
            inner: Utf8Error(s),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BtmError {
    pub code: String,
    pub msg: String,
    pub error_detail: String,
}

impl std::error::Error for BtmError {}

impl Display for BtmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let errmsg = format!(
            r#"{{"code":"{}","msg":"{}","error_detail":"{}"}}"#,
            self.code, self.msg, self.error_detail
        );
        write!(f, "{}", errmsg)
    }
}

impl BtmError {
    pub fn new<T>(response: Response<T>) -> Self
    where
        T: Serialize,
    {
        Self {
            code: response.code,
            msg: response.msg,
            error_detail: response.detail,
        }
    }
}
