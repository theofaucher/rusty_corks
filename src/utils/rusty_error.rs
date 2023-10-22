use std::error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::sync::mpsc::TryRecvError;

use macroquad::prelude::FileError;

// Definition of a custom result alias to simplify error handling.
pub type RustyResult<T> = Result<T, RustyError>;

// Definition of a custom locking error with an associated message.
// This error is used in the game to handle mutex locking error.
#[derive(Debug)]
pub struct LockError {
    pub message: String,
}

// Definition of an error related to directory reading with an associated message.
#[derive(Debug)]
pub struct ReadDirectoryError {
    pub message: String,
}

// Custom error type, to simplify error handling
// It is a wrapper around the different error types
#[derive(Debug)]
pub enum RustyError {
    RustyLock(LockError),
    File(FileError),
    Recv(TryRecvError),
    ReadDirectory(std::io::Error),
    LaneNotFound,
}

// Implementation of the `Display` trait to display errors in a user-friendly way.
impl Display for RustyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RustyError::RustyLock(e) =>
                write!(f, "Rusty lock error: {}", e.message),
            RustyError::LaneNotFound =>
                write!(f, "Lane not found"),
            _ => Ok(()),
        }
    }
}

// Implementation of the `Error` trait to handle error sources.
impl error::Error for RustyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RustyError::RustyLock(_) => None,
            RustyError::File(ref e) => Some(e),
            RustyError::Recv(ref e) => Some(e),
            RustyError::ReadDirectory(ref e) => Some(e),
            RustyError::LaneNotFound => None,
        }
    }
}

// Conversion of various errors into `RustyError` using `From` implementations.
impl From<FileError> for RustyError {
    fn from(err: FileError) -> RustyError {
        RustyError::File(err)
    }
}

impl From<TryRecvError> for RustyError {
    fn from(err: TryRecvError) -> RustyError {
        RustyError::Recv(err)
    }
}

impl From<std::io::Error> for RustyError {
    fn from(err: std::io::Error) -> RustyError {
        RustyError::ReadDirectory(err)
    }
}