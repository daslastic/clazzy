use crate::data::clazz::ClazzError;
use confy::ConfyError;
use log::SetLoggerError;
use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug)]
pub enum ProgramError {
    ClazzError(ClazzError),
    DeserializationError(ConfyError),
    SetLogger(SetLoggerError),
    Notify(notify_rust::error::Error),
    Kill(&'static str),
}

impl Error for ProgramError {}
impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgramError::ClazzError(e) => write!(f, "Clazz error: {}", e),
            ProgramError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            ProgramError::SetLogger(e) => write!(f, "Logger error: {}", e),
            ProgramError::Notify(e) => write!(f, "Sending notification failed: {}", e),
            ProgramError::Kill(s) => write!(f, "Failed to kill process '{}'", s),
        }
    }
}

impl From<ClazzError> for ProgramError {
    fn from(e: ClazzError) -> Self {
        return ProgramError::ClazzError(e);
    }
}

impl From<ConfyError> for ProgramError {
    fn from(e: ConfyError) -> Self {
        return ProgramError::DeserializationError(e);
    }
}

impl From<SetLoggerError> for ProgramError {
    fn from(e: SetLoggerError) -> Self {
        return ProgramError::SetLogger(e);
    }
}

impl From<notify_rust::error::Error> for ProgramError {
    fn from(e: notify_rust::error::Error) -> Self {
        return ProgramError::Notify(e);
    }
}

pub fn runtime_error(error: ProgramError) {
    match error {
        ProgramError::Notify(e) => log::error!("{}", e),
        ProgramError::Kill(e) => log::error!("{}", e),
        _ => {}
    }
}