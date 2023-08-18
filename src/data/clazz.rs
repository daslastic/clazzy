use crate::{data::raw_clazz::RawClazz, ClazzTool, ProgramError};
use chrono::{NaiveDate, NaiveTime, ParseError, Weekday};
use chrono_tz::Tz;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct Clazz {
    pub semesters: Vec<Semestery>,
    pub time_zone: Tz,
}

#[derive(Debug, Clone)]
pub struct Semestery {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub classes: Vec<Class>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub tool: ClazzTool,
    pub name: String,
    pub password: String,
    pub online: bool,
    pub instructors: Vec<String>,
    pub credits: f32,
    pub dates: Vec<Datey>,
}

#[derive(Debug, Clone)]
pub struct Datey {
    pub day: Weekday,
    pub from: NaiveTime,
    pub to: NaiveTime,
}

pub fn make(raw_clazz: RawClazz) -> Result<Clazz, ProgramError> {
    let mut semesters: Vec<Semestery> = Vec::new();

    let time_zone: Tz = match raw_clazz.time_zone.parse() {
        Ok(t) => t,
        Err(e) => return Err(ClazzError::ParseTimezone(e))?,
    };

    for raw_semester in raw_clazz.semesters.iter() {
        let mut classes: Vec<Class> = Vec::new();

        for raw_class in raw_semester.classes.iter() {
            let mut dates: Vec<Datey> = Vec::new();

            for raw_date in raw_class.dates.iter() {
                dates.push(Datey {
                    day: raw_date.day,
                    from: match NaiveTime::parse_from_str(&raw_date.from, "%H:%M") {
                        Ok(s) => s,
                        Err(e) => {
                            return Err(ClazzError::ParseClassTime(raw_date.from.clone(), e))?
                        }
                    },
                    to: match NaiveTime::parse_from_str(&raw_date.to, "%H:%M") {
                        Ok(s) => s,
                        Err(e) => return Err(ClazzError::ParseClassTime(raw_date.to.clone(), e))?,
                    },
                })
            }

            classes.push(Class {
                tool: raw_class.tool.clone(),
                name: raw_class.name.clone(),
                password: raw_class.password.clone(),
                online: raw_class.online,
                instructors: raw_class.instructors.clone(),
                credits: raw_class.credits,
                dates,
            });
        }

        semesters.push(Semestery {
            to: match NaiveDate::parse_from_str(&raw_semester.to, "%b %d, %Y") {
                Ok(s) => s,
                Err(e) => return Err(ClazzError::ParseSemTime(raw_semester.to.clone(), e))?,
            },
            from: match NaiveDate::parse_from_str(&raw_semester.from, "%b %d, %Y") {
                Ok(s) => s,
                Err(e) => return Err(ClazzError::ParseSemTime(raw_semester.from.clone(), e))?,
            },
            classes,
        })
    }

    Ok(Clazz {
        time_zone,
        semesters,
    })
}

#[derive(Clone, Debug)]
pub enum ClazzError {
    ParseSemTime(String, ParseError),
    ParseClassTime(String, ParseError),
    ParseTimezone(String),
}

impl Error for ClazzError {}

impl fmt::Display for ClazzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ClazzError::ParseSemTime(s, e) => {
                write!(f, "Invalid semester time: {}. Error: {}.", s, e)
            }
            ClazzError::ParseClassTime(s, e) => {
                write!(f, "Invalid class time: {}. Error: {}.", s, e)
            }
            ClazzError::ParseTimezone(e) => {
                write!(f, "Invalid timezone: {}.", e)
            }
        }
    }
}
