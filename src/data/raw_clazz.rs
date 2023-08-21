use crate::ClazzTool;
use chrono::Weekday;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawClazz {
    pub semesters: Vec<Semester>,
    pub time_zone: String,
    pub notify_sound: Option<String>,
    pub warn_minutes: Option<i32>,
}

impl Default for RawClazz {
    fn default() -> Self {
        Self {
            semesters: Vec::new(),
            time_zone: String::from("America/New_York"),
            notify_sound: None,
            warn_minutes: None,
        }
    }
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