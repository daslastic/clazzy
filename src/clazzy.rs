use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::data::clazz::{Clazz, Semestery};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClazzTool {
    Zoom,
    GoogleMeets,
}

#[derive(Clone)]
pub struct Clazzy {
    pub clazz: Clazz,
    pub sem_id: Option<usize>,
}

impl Clazzy {
    pub fn new(clazz: Clazz) -> Self {
        let mut s = Self {
            clazz,
            sem_id: None,
        };

        s.sem_id = is_semester(&s);
        s
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