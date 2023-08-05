use crate::{app::ClazzTool, raw_clazz::RawClazz};
use chrono::{NaiveDate, NaiveTime, ParseError, Weekday};
use std::fmt;

#[derive(Debug)]
pub struct Clazzy {
    pub sem_id: usize,
    pub semesters: Vec<Semestery>,
}

#[derive(Debug)]
pub struct Semestery {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub clazzes: Vec<Clazz>,
}

#[derive(Debug)]
pub struct Clazz {
    pub tool: ClazzTool,
    pub name: String,
    pub password: String,
    pub online: bool,
    pub instructor: Option<String>,
    pub instructors: Option<Vec<String>>,
    pub credits: f32,
    pub dates: Vec<Datey>,
}

#[derive(Debug)]
pub struct Datey {
    // day: Weekday,
    pub from: NaiveTime,
    pub to: NaiveTime,
    pub instructors: Option<Vec<i32>>,
    pub instructor: Option<String>,
    pub tool: Option<ClazzTool>,
}

pub fn make_clazzy(raw_clazz: RawClazz) -> Result<Clazzy, InitProgramError> {
    if raw_clazz.sem_id > raw_clazz.semesters.len() - 1 {
        return Err(InitProgramError::IndexSemError(raw_clazz.sem_id));
    }

    let mut semesters: Vec<Semestery> = Vec::new();

    for semester in raw_clazz.semesters.iter() {
        let mut clazzes: Vec<Clazz> = Vec::new();

        for clazz in semester.clazzes.iter() {
            let mut dates: Vec<Datey> = Vec::new();

            // for dates in clazz.dates.iter() {
            //     dates.
            // }

            clazzes.push(Clazz {
                tool: clazz.tool.clone(),
                name: clazz.name.clone(),
                password: clazz.password.clone(),
                online: clazz.online.clone(),
                instructor: clazz.instructor.clone(),
                instructors: clazz.instructors.clone(),
                credits: clazz.credits,
                dates,
            });
        }

        semesters.push(Semestery {
            to: match NaiveDate::parse_from_str(&semester.to, "%b %d, %Y") {
                Ok(s) => s,
                Err(e) => return Err(InitProgramError::ParseSemTime(semester.to.clone(), e)),
            },
            from: match NaiveDate::parse_from_str(&semester.from, "%b %d, %Y") {
                Ok(s) => s,
                Err(e) => return Err(InitProgramError::ParseSemTime(semester.from.clone(), e)),
            },
            clazzes,
        })
    }

    Ok(Clazzy {
        sem_id: raw_clazz.sem_id,
        semesters,
    })
}

#[derive(Clone, Debug)]
pub enum InitProgramError {
    IndexSemError(usize),
    ParseSemTime(String, ParseError),
}

impl fmt::Display for InitProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            InitProgramError::IndexSemError(sem_id) => {
                write!(f, "sem_id: {}, is invalid.", sem_id)
            }
            InitProgramError::ParseSemTime(s, e) => {
                write!(f, "Invalid semester time: {}. Error: {}", s, e)
            }
        }
    }
}
