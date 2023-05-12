use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum MyError {
    NotAnEchoRequest,
    IoErr(io::Error),
    WriteError(etherparse::WriteError),
    ReadError(etherparse::ReadError),
}

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::IoErr(e)
    }
}

impl From<etherparse::WriteError> for MyError {
    fn from(e: etherparse::WriteError) -> Self {
        MyError::WriteError(e)
    }
}

impl From<etherparse::ReadError> for MyError {
    fn from(e: etherparse::ReadError) -> Self {
        MyError::ReadError(e)
    }
}
