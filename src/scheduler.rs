use std::sync::{Arc, Mutex};

use crate::{
    clazzy::{self},
    data::clazz::Datey,
    notification, Clazzy, DatePos, ProgramError,
};

use clokwerk::*;
use std::{thread, time::Duration};

pub fn start(clazzy_ref: Arc<Mutex<Clazzy>>) -> Result<(), ProgramError> {
    let mut scheduler;
    let dates: Vec<(Datey, DatePos)>;

    {
        let mut clazzy = clazzy_ref.lock().unwrap();
        // print_schedule(&mut clazzy);
        scheduler = Scheduler::with_tz(clazzy.clazz.time_zone);
        dates = clazzy::init_dates(&mut clazzy);
    }

    for date in dates.iter() {
        let day = clazzy::into_interval(date.0.day);
        let id = date.1;

        if let Some(warn) = clazzy_ref.lock().unwrap().clazz.warn_minutes {
            let clazzy_ref = clazzy_ref.clone();
            let new_date = date.0.from - chrono::Duration::minutes(warn.into());
            scheduler.every(day).at_time(new_date).run(move || {
                clazzy::warn_class(&mut clazzy_ref.lock().unwrap(), id, warn);
            });
        }

        {
            let clazzy_ref = clazzy_ref.clone();
            scheduler.every(day).at_time(date.0.from).run(move || {
                clazzy::start_class(&mut clazzy_ref.lock().unwrap(), id);
            });
        }

        {
            let clazzy_ref = clazzy_ref.clone();
            scheduler.every(day).at_time(date.0.to).run(move || {
                clazzy::end_class(&mut clazzy_ref.lock().unwrap(), id);
            });
        }
    }

    {
        let clazzy_ref = clazzy_ref.clone();
        scheduler.every(10.seconds()).run(move || {
            let mut clazzy = clazzy_ref.lock().unwrap();
            notification::process_messege(&mut clazzy);
        });
    }

    // checks if there is a new active semester
    // triggers reset
    {
        let clazzy_ref = clazzy_ref.clone();
        scheduler.every(Interval::Weekday).run(move || {
            let mut clazzy = clazzy_ref.lock().unwrap();
            if clazzy.sem_id != clazzy::is_semester(&clazzy) {
                clazzy.reset = true;
            }
        });
    }

    loop {
        if clazzy::is_reset(&mut clazzy_ref.clone().lock().unwrap()) {
            return Ok(start(clazzy_ref.clone())?);
        }

        scheduler.run_pending();
        thread::sleep(Duration::from_secs(10));
    }
}

// pub fn print_schedule(clazzy: &mut Clazzy) {}
