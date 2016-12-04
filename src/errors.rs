use std::error::Error;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum WikiError {
    Parse(io::Error),
}

impl fmt::Display for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WikiError::Parse(ref err) => err.fmt(f),
        }
    }
}

impl Error for WikiError {
    fn description(&self) -> &str {
        match *self {
            WikiError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            WikiError::Parse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for WikiError {
    fn from(err: io::Error) -> WikiError {
        WikiError::Parse(err)
    }
}

