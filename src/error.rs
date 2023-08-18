use crate::data::{clazz::ClazzError, raw_clazz::DeserializationError};
use log::SetLoggerError;
use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug)]
pub enum ProgramError {
    ClazzError(ClazzError),
    DeserializationError(DeserializationError),
    SetLogger(SetLoggerError),
}

impl Error for ProgramError {}
impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgramError::ClazzError(e) => write!(f, "Clazz error: {}", e),
            ProgramError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            ProgramError::SetLogger(e) => write!(f, "Logger error: {}", e),
        }
    }
}

impl From<ClazzError> for ProgramError {
    fn from(e: ClazzError) -> Self {
        return ProgramError::ClazzError(e);
    }
}

impl From<DeserializationError> for ProgramError {
    fn from(e: DeserializationError) -> Self {
        return ProgramError::DeserializationError(e);
    }
}

impl From<SetLoggerError> for ProgramError {
    fn from(e: SetLoggerError) -> Self {
        return ProgramError::SetLogger(e);
    }
}