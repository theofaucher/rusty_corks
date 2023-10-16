use std::error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::sync::mpsc::TryRecvError;

use macroquad::prelude::FileError;

pub type RustyResult<T> = Result<T, RustyError>;

#[derive(Debug)]
pub struct LockError {
    pub message: String,
}

#[derive(Debug)]
pub struct ReadDirectoryError {
    pub message: String,
}

#[derive(Debug)]
pub enum RustyError {
    RustyLock(LockError),
    File(FileError),
    Recv(TryRecvError),
    ReadDirectory(std::io::Error),
    LaneNotFound,
}

impl Display for RustyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RustyError::RustyLock(e) =>
                write!(f, "Rusty lock error: {}", e.message),
            RustyError::LaneNotFound =>
                write!(f, "Lane not found"),
            // RustyError::ReadDirectory(e) =>
            //     write!(f, "Read directory error: {}", e.message),
            _ => Ok(()),
        }
    }
}

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