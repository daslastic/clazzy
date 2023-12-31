use std::cell::RefCell;

use chrono::{Local, Weekday};
use clokwerk::Interval;
use notify_rust::Notification;

use crate::{
    config::{Class, Clazz, Datey},
    ProgramError,
};

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
    pub current_class: Option<RefCell<CurrentClass>>,
    pub reset: bool,
    notifications: Vec<Notification>,
}

impl Clazzy {
    pub fn new(clazz: Clazz) -> Self {
        Self {
            clazz,
            sem_id: None,
            current_class: None,
            notifications: Vec::new(),
            reset: false,
        }
    }

    pub fn start_class(&mut self, id: DatePos) {
        if let Some(current_class) = self.current_class.clone() {
            let class = &self.clazz.semesters[id.0].classes[id.1];
            log::info!(
                "Class '{}' couldn't start! Because '{}' is active.",
                &class.name,
                &current_class.borrow().class.name,
            );
        } else {
            let class = &self.clazz.semesters[id.0].classes[id.1];
            self.current_class =
                Some(CurrentClass::new(class.clone(), class.dates[id.2].clone()).into());
            log::info!("Class '{}' started!", &class.name);
        }
    }

    pub fn end_class(&mut self, id: DatePos) {
        if let Some(current_class) = self.current_class.clone() {
            let class = &self.clazz.semesters[id.0].classes[id.1];
            // ensure that no overlap will introduce bugs
            if current_class.borrow().class.name == class.name {
                log::info!("Class '{}' ended!", class.name);
                self.send_class_messege(format!("Class has ended"));
                self.current_class = None;
            }
        }
    }

    pub fn warn_class(&mut self, id: DatePos, time: i32) {
        let class_name = self.clazz.semesters[id.0].classes[id.1].name.clone();
        self.send_messege(
            class_name.clone(),
            format!("Will begin in {} minutes", time),
        );
        log::info!("Class '{}' will begin in '{}' minutes.", class_name, time);
    }

    pub fn is_semester(&self) -> Option<usize> {
        let now = Local::now().naive_local().date();
        for (i, sem) in self.clazz.semesters.iter().enumerate() {
            if now >= sem.from && now <= sem.to {
                return Some(i);
            }
        }
        None
    }

    pub fn is_reset(&mut self) -> bool {
        if self.reset {
            self.current_class = None;
            self.reset = false;
            true
        } else {
            false
        }
    }

    pub fn send_messege(&mut self, title: String, msg: String) {
        let mut notification = Notification::new().summary(&title).body(&msg).clone();
        if let Some(name) = &self.clazz.notify_sound {
            notification.sound_name(name);
        }
        self.notifications.push(notification);
    }

    pub fn send_class_messege(&mut self, msg: String) {
        if let Some(current_class) = self.current_class.clone() {
            self.send_messege(current_class.borrow().class.name.clone(), msg);
        }
    }

    pub fn process_next_messege(&mut self) {
        if !self.notifications.is_empty() {
            let notification = &self.notifications[0];
            if let Err(e) = notification.show() {
                crate::error::runtime_error(ProgramError::Notify(e));
            }
            self.notifications.remove(0);
        }
    }
}

pub fn into_interval(weekday: Weekday) -> Interval {
    match weekday {
        Weekday::Mon => Interval::Monday,
        Weekday::Tue => Interval::Tuesday,
        Weekday::Wed => Interval::Wednesday,
        Weekday::Thu => Interval::Thursday,
        Weekday::Fri => Interval::Friday,
        Weekday::Sat => Interval::Saturday,
        Weekday::Sun => Interval::Sunday,
    }
}

pub fn into_weekday(num: usize) -> Option<Weekday> {
    match num {
        0 => Some(Weekday::Mon),
        1 => Some(Weekday::Tue),
        2 => Some(Weekday::Wed),
        3 => Some(Weekday::Thu),
        4 => Some(Weekday::Fri),
        5 => Some(Weekday::Sat),
        6 => Some(Weekday::Sun),
        _ => None,
    }
}
