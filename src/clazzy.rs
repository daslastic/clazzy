use crate::domeplz::{ClazzTool, Weekday};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Clazzy {
    sem_id: i32,
    time_zone: String,
    semesters: Vec<Semester>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Semester {
    student_name: String,
    from: String,
    to: String,
    clazzes: Vec<Clazz>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Clazz {
    tool: ClazzTool,
    name: String,
    password: String,
    online: bool,
    instructor: Option<String>,
    instructors: Option<Vec<String>>,
    credits: f32,
    dates: Vec<Date>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Date {
    day: Weekday,
    from: String,
    to: String,
    instructors: Option<Vec<i32>>,
}

#[derive(Debug)] // sugar dady
pub enum DeserializationError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
    Idiot(ron::error::Error),
}

pub fn make_her(fpath: String) -> Result<Clazzy, DeserializationError> {
    let file = File::open(fpath).map_err(DeserializationError::Io)?;
    let clazzy = from_reader(file).map_err(DeserializationError::Ron)?;

    // println!(
    //     "Hot RON: {}",
    //     ron::ser::to_string_pretty(&clazzy, ron::ser::PrettyConfig::default())
    //         .map_err(DeserializationError::Idiot)?,
    // );

    println!(
        "RON: {}",
        ron::to_string(&clazzy).map_err(DeserializationError::Idiot)?
    );

    Ok(clazzy)
}
