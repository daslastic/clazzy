use crate::ClazzTool;
use chrono::Weekday;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, fs::File};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawClazz {
    pub semesters: Vec<Semester>,
    pub time_zone: String,
    pub notify_sound: Option<String>,
    pub warn_minutes: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Semester {
    pub from: String,
    pub to: String,
    pub classes: Vec<Class>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Class {
    pub tool: ClazzTool,
    pub name: String,
    pub dates: Vec<Date>,
    pub instructors: Vec<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Date {
    pub day: Weekday,
    pub from: String,
    pub to: String,
}

pub fn serialize_her(fpath: String) -> Result<RawClazz, DeserializationError> {
    let file = File::open(fpath).map_err(DeserializationError::Io)?;
    let raw_clazz = from_reader(file).map_err(DeserializationError::Ron)?;
    Ok(raw_clazz)
}

#[derive(Debug)] // sugar dady
pub enum DeserializationError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
    Idiot(ron::error::Error),
}

impl Error for DeserializationError {}

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DeserializationError::Io(e) => {
                write!(f, "Config not found: {}", e)
            }
            DeserializationError::Ron(e) => {
                write!(f, "Syntax error: {}", e)
            }
            DeserializationError::Idiot(e) => {
                write!(f, "bruh how: {}", e)
            }
        }
    }
}