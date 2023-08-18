use serde::{Deserialize, Serialize};

use chrono::{Local, Weekday};
use clokwerk::Interval;

use crate::data::clazz::{Class, Clazz, Datey};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClazzTool {
    Zoom,
    GoogleMeets,
}

#[derive(Debug, Clone)]
pub enum ClassState {
    Unknown,
    Started,
    Joining,
    Ended,
}

#[derive(Debug)]
pub struct CurrentClass {
    pub class: Class,
    pub date: Datey,
    pub state: ClassState,
}

impl CurrentClass {
    pub fn new(class: Class, date: Datey) -> Self {
        Self {
            class,
            date,
            state: ClassState::Started,
        }
    }
}

pub struct Clazzy {
    pub clazz: Clazz,
    pub sem_id: Option<usize>,
    pub current_class: Option<CurrentClass>,
}

impl Clazzy {
    pub fn new(clazz: Clazz) -> Self {
        Self {
            clazz,
            sem_id: None,
            current_class: None,
        }
    }
}

pub fn is_semester(clazzy: &Clazzy) -> Option<usize> {
    let now = Local::now().naive_local().date();
    for (i, sem) in clazzy.clazz.semesters.iter().enumerate() {
        if now >= sem.from && now <= sem.to {
            if clazzy.sem_id.is_none() || clazzy.sem_id.is_some() && i != clazzy.sem_id.unwrap() {
                log::info!("Semester {}, is active. ({}/{})", i, sem.from, sem.to);
            }
            return Some(i);
        }
    }
    log::info!("No semester applies to today :)");
    None
}

pub fn start_class(clazzy: &mut Clazzy, id: (usize, usize, usize)) {
    match &clazzy.current_class {
        Some(current_class) => {
            let class = &clazzy.clazz.semesters[id.0].classes[id.1];
            log::info!(
                "Class '{}' couldn't start! Because '{}' is active.",
                class.name,
                current_class.class.name,
            );
        }
        _ => {
            let class = &mut clazzy.clazz.semesters[id.0].classes[id.1];
            log::info!("Class '{}' started!", class.name);
            clazzy.current_class =
                Some(CurrentClass::new(class.clone(), class.dates[id.2].clone()));
        }
    }
}

pub fn end_class(clazzy: &mut Clazzy, id: (usize, usize, usize)) {
    match &clazzy.current_class {
        Some(current_class) => {
            let class = &clazzy.clazz.semesters[id.0].classes[id.1];
            // ensure that no overlap will introduce bugs
            if current_class.class.name == class.name {
                log::info!("Class '{}' ended!", class.name);
                clazzy.current_class = None;
            }
        }
        _ => {}
    }
}

pub fn process_class(clazzy: &mut Clazzy) {
    if let Some(class) = &mut clazzy.current_class {
        log::info!("Class '{}' is active!", class.class.name);
    }
}

pub fn into_interval(weekday: Weekday) -> Interval {
    return match weekday {
        Weekday::Mon => Interval::Monday,
        Weekday::Tue => Interval::Tuesday,
        Weekday::Wed => Interval::Wednesday,
        Weekday::Thu => Interval::Thursday,
        Weekday::Fri => Interval::Friday,
        Weekday::Sat => Interval::Saturday,
        Weekday::Sun => Interval::Sunday,
    };
}
