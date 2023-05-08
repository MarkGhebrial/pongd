use std::io;
use std::error::Error;
use std::fmt::{Display, Formatter, self};

use etherparse::WriteError;

#[derive(Debug)]
pub enum MyError {
    NotAnEchoRequest,
    IoErr(io::Error),
    WriteError(WriteError),
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

impl From<WriteError> for MyError {
    fn from(e: WriteError) -> Self {
        MyError::WriteError(e)
    }
}