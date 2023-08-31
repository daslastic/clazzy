use std::sync::{Arc, Mutex};

use crate::{
    clazzy::{self},
    pretty_print, Clazzy, DatePos, ProgramError,
};

use chrono::{Datelike, Local};
use clokwerk::*;
use std::{thread, time::Duration};

pub fn start(clazzy_ref: Arc<Mutex<Clazzy>>) -> Result<(), ProgramError> {
    let mut scheduler;

    {
        let mut clazzy = clazzy_ref.lock().unwrap();
        scheduler = Scheduler::with_tz(clazzy.clazz.time_zone);
        clazzy.sem_id = clazzy.is_semester();

        pretty_print::sexy(&mut clazzy);

        if let Some(sem_id) = clazzy.sem_id {
            let local = Local::now().with_timezone(&clazzy.clazz.time_zone);
            let current_time = local.time();
            let current_weekday = local.date_naive().weekday();
            let mut late_to: Option<(String, DatePos)> = None;
            for (c, class) in clazzy.clazz.semesters[sem_id].classes.iter().enumerate() {
                for (d, date) in class.dates.iter().enumerate() {
                    let day = clazzy::into_interval(date.day);
                    let id = (sem_id, c, d);

                    {
                        if let Some(warn) = clazzy.clazz.warn_minutes {
                            let clazzy_ref = clazzy_ref.clone();
                            let new_date = date.from - chrono::Duration::minutes(warn.into());
                            scheduler.every(day).at_time(new_date).run(move || {
                                clazzy_ref.lock().unwrap().warn_class(id, warn);
                            });
                        }
                    }

                    {
                        let clazzy_ref = clazzy_ref.clone();
                        scheduler.every(day).at_time(date.from).run(move || {
                            clazzy_ref.lock().unwrap().start_class(id);
                        });
                    }

                    {
                        let clazzy_ref = clazzy_ref.clone();
                        scheduler.every(day).at_time(date.to).run(move || {
                            clazzy_ref.lock().unwrap().end_class(id);
                        });
                    }

                    if current_time >= date.from
                        && current_time <= date.to
                        && date.day == current_weekday
                    {
                        if late_to.is_some() {
                            log::info!("You are also late to: '{}'", class.name);
                        } else {
                            late_to = Some((class.name.clone(), id));
                        }
                    }
                }
            }

            if let Some(late_to) = late_to {
                clazzy.send_messege(
                    late_to.0.clone(),
                    format!("You are late, class has started."),
                );
                log::info!("You are late to class '{}'", late_to.0);
                clazzy.start_class(late_to.1);
            }

            {
                let clazzy_ref = clazzy_ref.clone();
                scheduler.every(5.seconds()).run(move || {
                    let mut clazzy = clazzy_ref.lock().unwrap();
                    clazzy.process_next_messege();
                });
            }
        }
    }

    // checks if there is a new active semester
    // triggers reset
    {
        let clazzy_ref = clazzy_ref.clone();
        scheduler.every(Interval::Weekday).run(move || {
            let mut clazzy = clazzy_ref.lock().unwrap();
            if clazzy.sem_id != clazzy.is_semester() {
                clazzy.reset = true;
            }
        });
    }

    loop {
        if clazzy_ref.clone().lock().unwrap().is_reset() {
            return Ok(start(clazzy_ref.clone())?);
        }

        scheduler.run_pending();
        thread::sleep(Duration::from_secs(1));
    }
}
