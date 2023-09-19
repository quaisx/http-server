use crate::http::method::MethodError;
use std::error::Error;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

pub enum HandleError {
    ErrEncoding,
    ErrMethod,
    ErrProtocol,
    ErrRequest,
}

impl HandleError {
    fn msg(&self) -> &str {
        match self {
            Self::ErrRequest => "Invalid Request",
            Self::ErrEncoding => "Invalid Encoding",
            Self::ErrProtocol => "Invalid Protocol",
            Self::ErrMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for HandleError {
    fn from(_: MethodError) -> Self {
        Self::ErrMethod
    }
}

impl From<Utf8Error> for HandleError {
    fn from(_: Utf8Error) -> Self {
        Self::ErrEncoding
    }
}

impl Display for HandleError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg())
    }
}

impl Debug for HandleError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg())
    }
}

impl Error for HandleError {}