use crate::{scheduler::ProgramError, ClazzTool};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, fs::File};

use chrono::Weekday;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawClazz {
    pub semesters: Vec<Semester>,
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
    pub code: String,
    pub password: String,
    pub online: bool,
    pub credits: f32,
    pub dates: Vec<Date>,
    pub instructors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Date {
    pub day: Weekday,
    pub from: String,
    pub to: String,
}

pub fn serialize_her(fpath: String) -> Result<RawClazz, ProgramError> {
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
                write!(
                    f,
                    "Wow, your either bad at cd or r stupid. Guess which one(s): {}",
                    e
                )
            }
            DeserializationError::Ron(e) => {
                write!(
                    f,
                    "Wow, your either bad at cd or r stupid. Guess which one(s): {}",
                    e
                )
            }
            DeserializationError::Idiot(e) => {
                write!(f, "bruh how: {}", e)
            }
        }
    }
}