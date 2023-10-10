use std::error;
use std::fmt;
use std::fmt::Debug;
use std::sync::mpsc::TryRecvError;

use macroquad::prelude::FileError;

pub type RustyResult<T> = Result<T, RustyError>;

#[derive(Debug)]
pub enum RustyError {
    RustyLock,
    File(FileError),
    Recv(TryRecvError),
}

impl fmt::Display for RustyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RustyError::RustyLock =>
                write!(f, "Rusty lock error"),
            _ => Ok(()),
        }
    }
}

impl error::Error for RustyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RustyError::RustyLock => None,
            RustyError::File(ref e) => Some(e),
            RustyError::Recv(ref e) => Some(e),
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