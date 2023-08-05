use crate::app::ClazzTool;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

use chrono::Weekday;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawClazz {
    pub sem_id: usize,
    pub semesters: Vec<Semester>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Semester {
    pub from: String,
    pub to: String,
    pub clazzes: Vec<Clazz>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clazz {
    pub tool: ClazzTool,
    pub name: String,
    pub code: String,
    pub password: String,
    pub online: bool,
    pub credits: f32,
    pub dates: Vec<Date>,
    pub instructor: Option<String>,
    pub instructors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Date {
    // day: Weekday,
    from: String,
    to: String,
    instructors: Option<Vec<i32>>,
    instructor: Option<String>,
    tool: Option<ClazzTool>,
}

#[derive(Debug)] // sugar dady
pub enum DeserializationError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
    Idiot(ron::error::Error),
}

pub fn serialize_her(fpath: String) -> Result<RawClazz, DeserializationError> {
    let file = File::open(fpath).map_err(DeserializationError::Io)?;
    let clazzy = from_reader(file).map_err(DeserializationError::Ron)?;

    // println!(
    //     "Hot RON: {}",
    //     ron::ser::to_string_pretty(&clazzy, ron::ser::PrettyConfig::default())
    //         .map_err(DeserializationError::Idiot)?,
    // );
    // println!(
    //     "RON: {}",
    //     ron::to_string(&clazzy).map_err(DeserializationError::Idiot)?
    // );

    Ok(clazzy)
}
