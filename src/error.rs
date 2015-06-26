use std::fmt;
use rustc_serialize::json;
/// Telegram-Bot Result
pub type Result<T> = ::std::result::Result<T, Error>;

/// Telegram-Bot Error.
#[derive(Debug)]
pub enum Error {
    Http(::hyper::error::Error),
    Io(::std::io::Error),
    JsonDecode(json::DecoderError),
    JsonEncode(json::EncoderError),
    Api(String),
    InvalidState(String),
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::JsonDecode(ref e) => e.description(),
            Error::JsonEncode(ref e) => e.description(),
            Error::Api(ref s) => &*s,
            Error::InvalidState(ref s) => &*s,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => e.fmt(f),
            Error::Io(ref e) => e.fmt(f),
            Error::JsonDecode(ref e) => e.fmt(f),
            Error::JsonEncode(ref e) => e.fmt(f),
            Error::Api(ref s) => s.fmt(f),
            Error::InvalidState(ref s) => s.fmt(f),
        }
    }
}

macro_rules! from_impl {
    ($ty:path, $variant:ident) => (
        impl From<$ty> for Error {
            fn from(e: $ty) -> Self {
                Error::$variant(e)
            }
        }
    )
}

from_impl!(::hyper::error::Error, Http);
from_impl!(::std::io::Error, Io);
from_impl!(json::DecoderError, JsonDecode);
from_impl!(json::EncoderError, JsonEncode);
