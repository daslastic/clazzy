use std::process::Command;

use chrono::{Local, Weekday};
use clokwerk::Interval;
use serde::{Deserialize, Serialize};

use crate::{
    data::clazz::{Class, Clazz, Datey},
    notification,
};
use notify_rust::Notification;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClazzTool {
    Zoom,
    Teams,
}

pub type DatePos = (usize, usize, usize);

#[derive(Debug, Clone)]
pub struct CurrentClass {
    pub class: Class,
    pub date: Datey,
}

impl CurrentClass {
    pub fn new(class: Class, date: Datey) -> Self {
        Self { class, date }
    }
}

pub struct Clazzy {
    pub clazz: Clazz,
    pub sem_id: Option<usize>,
    pub current_class: Option<CurrentClass>,
    pub reset: bool,
    pub notifications: Vec<Notification>,
}

impl Clazzy {
    pub fn new(clazz: Clazz) -> Self {
        Self {
            clazz,
            sem_id: None,
            current_class: None,
            reset: false,
            notifications: Vec::new(),
        }
    }
}

pub fn init_dates(clazzy: &mut Clazzy) -> Vec<(Datey, DatePos)> {
    let mut missing: Option<(String, DatePos)> = None;
    let mut dates: Vec<(Datey, DatePos)> = Vec::new();
    if let Some(sem_id) = is_semester(clazzy) {
        let current_time = Local::now().with_timezone(&clazzy.clazz.time_zone).time();

        for (i, class) in clazzy.clazz.semesters[sem_id].classes.iter().enumerate() {
            for (o, date) in class.dates.iter().enumerate() {
                let pos = (sem_id, i, o);
                dates.push((date.clone(), pos));

                if missing.is_none() && current_time >= date.from && current_time <= date.to {
                    missing = Some((class.name.clone(), pos));
                }
            }
        }

        if let Some(missing) = missing {
            notification::send_messege(
                clazzy,
                missing.0.clone(),
                format!("You are late, class has started."),
            );
            log::info!("You are late to class '{}'", missing.0);
            start_class(clazzy, missing.1);
        }
    }

    dates
}

pub fn start_class(clazzy: &mut Clazzy, id: DatePos) {
    match &clazzy.current_class {
        Some(current_class) => {
            let class = &clazzy.clazz.semesters[id.0].classes[id.1];
            log::info!(
                "Class '{}' couldn't start! Because '{}' is active.",
                &class.name,
                &current_class.class.name,
            );
        }
        _ => {
            let class = &clazzy.clazz.semesters[id.0].classes[id.1];
            clazzy.current_class =
                Some(CurrentClass::new(class.clone(), class.dates[id.2].clone()));
            log::info!("Class '{}' started!", &class.name);
            join_class(clazzy);
        }
    }
}

pub fn end_class(clazzy: &mut Clazzy, id: DatePos) {
    match clazzy.current_class.clone() {
        Some(current_class) => {
            let class = &clazzy.clazz.semesters[id.0].classes[id.1];
            // ensure that no overlap will introduce bugs
            if current_class.class.name == class.name {
                log::info!("Class '{}' ended!", class.name);
                notification::send_class_messege(clazzy, format!("Class has ended"));
                clazzy.current_class = None;

                match current_class.class.tool {
                    ClazzTool::Zoom => {
                        if let Err(_) = Command::new("pkill").arg("zoom.us").output() {
                            crate::error::runtime_error(crate::ProgramError::Kill("Zoom"));
                        }
                    }
                    ClazzTool::Teams => {
                        if let Err(_) = Command::new("pkill").arg("Teams").output() {
                            crate::error::runtime_error(crate::ProgramError::Kill("Teams"));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn warn_class(clazzy: &mut Clazzy, id: DatePos, time: i32) {
    let class_name = clazzy.clazz.semesters[id.0].classes[id.1].name.clone();
    notification::send_messege(
        clazzy,
        class_name.clone(),
        format!("Will begin in {} minutes", time),
    );
    log::info!("Class '{}' will begin in {} minutes.", class_name, time);
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

pub fn is_reset(clazzy: &mut Clazzy) -> bool {
    if clazzy.reset {
        clazzy.current_class = None;
        clazzy.reset = false;
        true
    } else {
        false
    }
}

pub fn join_class(clazzy: &mut Clazzy) {
    if let Some(current_class) = &mut clazzy.current_class {
        if let Some(url) = &current_class.class.url {
            match open::that(url) {
                Ok(_) => {
                    notification::send_class_messege(clazzy, format!("Class opened in browser"));
                    log::info!("Opened class in web browser");
                }
                Err(e) => {
                    notification::send_class_messege(
                        clazzy,
                        format!("Failed to open class! {}", e),
                    );
                    log::info!("Failed to open class");
                }
            }
        } else {
            notification::send_class_messege(
                clazzy,
                format!("Class has began, you will have to join manually."),
            );
        }
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
