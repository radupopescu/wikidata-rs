use std::error::Error;
use std::io;
use std::fmt;

use serde_json::Error as SerdeJSONError;

#[derive(Debug)]
pub enum WikiError {
    Parse(SerdeJSONError),
    Read(io::Error),
}

impl fmt::Display for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WikiError::Parse(ref err) => err.fmt(f),
            WikiError::Read(ref err) => err.fmt(f),
        }
    }
}

impl Error for WikiError {
    fn description(&self) -> &str {
        match *self {
            WikiError::Parse(ref err) => err.description(),
            WikiError::Read(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            WikiError::Parse(ref err) => Some(err),
            WikiError::Read(ref err) => Some(err),
        }
    }
}

impl From<SerdeJSONError> for WikiError {
    fn from(err: SerdeJSONError) -> WikiError {
        WikiError::Parse(err)
    }
}

impl From<io::Error> for WikiError {
    fn from(err: io::Error) -> WikiError {
        WikiError::Read(err)
    }
}

