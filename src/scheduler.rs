use std::sync::{Arc, Mutex};

use crate::{
    clazzy::{self},
    data::clazz::Datey,
    Clazzy, ProgramError,
};

use chrono::Local;
use clokwerk::*;
use std::{thread, time::Duration};

pub fn start(clazzy_ref: Arc<Mutex<Clazzy>>) -> Result<(), ProgramError> {
    let mut scheduler;
    let mut dates: Vec<(Datey, (usize, usize, usize))> = Vec::new();

    {
        let mut clazzy = clazzy_ref.try_lock().unwrap();
        let mut missing: Option<(usize, usize, usize)> = None;
        let sem_id = clazzy.sem_id.unwrap();

        scheduler = Scheduler::with_tz(clazzy.clazz.time_zone);

        let current_time = Local::now().with_timezone(&clazzy.clazz.time_zone).time();

        for (i, class) in clazzy.clazz.semesters[sem_id].classes.iter().enumerate() {
            for (o, date) in class.dates.iter().enumerate() {
                let pos = (sem_id, i, o);
                dates.push((date.clone(), pos));

                if current_time >= date.from && current_time <= date.to {
                    log::info!("You are missing class '{}'", class.name);
                    missing = Some(pos);
                }
            }

            log::info!("Class '{}' is setup!", class.name);
        }

        if let Some(missing) = missing {
            clazzy::start_class(&mut clazzy, missing);
        }
    }

    for date in dates {
        let day = clazzy::into_interval(date.0.day);
        let id = date.1;

        {
            let clazzy_ref = clazzy_ref.clone();
            scheduler.every(day).at_time(date.0.from).run(move || {
                let mut clazzy = clazzy_ref.try_lock().unwrap();
                clazzy::start_class(&mut clazzy, id);
            });
        }

        {
            let clazzy_ref = clazzy_ref.clone();
            scheduler.every(day).at_time(date.0.to).run(move || {
                let mut clazzy = clazzy_ref.try_lock().unwrap();
                clazzy::end_class(&mut clazzy, id);
            });
        }
    }

    {
        let clazzy_ref = clazzy_ref.clone();
        scheduler.every(1.minute()).run(move || {
            let mut clazzy = clazzy_ref.try_lock().unwrap();
            clazzy::process_class(&mut clazzy);
        });
    }

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}