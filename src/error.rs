use std::{io, string::FromUtf8Error};
use failure::Fail;

#[derive(Debug, Fail)]
pub enum PGError {
  /// IO Error
  #[fail(display = "{}",_0)]
  Io(#[cause] io::Error),
  /// serde error
  #[fail(display = "{}",_0)]
  Serde(#[cause] serde_yaml::Error),
  ///
  #[fail(display = "{}",_0)]
  StringError(String),
  ///
  #[fail(display = "UTF-8 error: {}",_0)]
  Utf8(#[cause] FromUtf8Error),
}

impl From<io::Error> for PGError {
    fn from(value: io::Error) -> Self {
        PGError::Io(value)
    }
}

impl From<serde_yaml::Error> for PGError {
    fn from(value: serde_yaml::Error) -> Self {
        PGError::Serde(value)
    }
}

impl From<FromUtf8Error> for PGError {
    fn from(value: FromUtf8Error) -> Self {
        PGError::Utf8(value)
    }
}

/// Result type for kvs 
pub type Result<T> = std::result::Result<T,PGError>;