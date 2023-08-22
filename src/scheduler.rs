use std::sync::{Arc, Mutex};

use crate::{
    clazzy::{self},
    data::clazz::Datey,
    notification, Clazzy, ProgramError,
};

use chrono::Local;
use clokwerk::*;
use std::{thread, time::Duration};

pub fn start(clazzy_ref: Arc<Mutex<Clazzy>>) -> Result<(), ProgramError> {
    let mut clazzy = clazzy_ref.lock().unwrap();
    clazzy.sem_id = clazzy::is_semester(&clazzy);
    let mut scheduler = Scheduler::with_tz(clazzy.clazz.time_zone);
    let current_time = Local::now().with_timezone(&clazzy.clazz.time_zone).time();
    print_schedule(&mut clazzy);

    if let Some(sem_id) = clazzy.sem_id {
        let mut is_late = false;
        for (c, class) in clazzy.clazz.semesters[sem_id].classes.iter().enumerate() {
            for (d, date) in class.dates.iter().enumerate() {
                let day = clazzy::into_interval(date.day);
                let id = (sem_id, c, d);

                if let Some(warn) = clazzy_ref.lock().unwrap().clazz.warn_minutes {
                    let clazzy_ref = clazzy_ref.clone();
                    let new_date = date.from - chrono::Duration::minutes(warn.into());
                    scheduler.every(day).at_time(new_date).run(move || {
                        clazzy::warn_class(&mut clazzy_ref.lock().unwrap(), id, warn);
                    });
                }

                {
                    let clazzy_ref = clazzy_ref.clone();
                    scheduler.every(day).at_time(date.from).run(move || {
                        clazzy::start_class(&mut clazzy_ref.lock().unwrap(), id);
                    });
                }

                {
                    let clazzy_ref = clazzy_ref.clone();
                    scheduler.every(day).at_time(date.to).run(move || {
                        clazzy::end_class(&mut clazzy_ref.lock().unwrap(), id);
                    });
                }

                let clazzy_ref = clazzy_ref.clone();
                let mut clazzy = clazzy_ref.lock().unwrap();
                if current_time >= date.from && current_time <= date.to {
                    notification::send_messege(
                        &mut clazzy,
                        class.name.clone(),
                        format!("You are late, class has started."),
                    );
                    log::info!("You are late to class '{}'", class.name);
                    if !is_late {
                        clazzy::start_class(&mut clazzy, id);
                        is_late = true;
                    }
                }
            }
        }

        {
            let clazzy_ref = clazzy_ref.clone();
            scheduler.every(8.seconds()).run(move || {
                let mut clazzy = clazzy_ref.lock().unwrap();
                notification::process_messege(&mut clazzy);
            });
        }
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

pub fn print_schedule(clazzy: &mut Clazzy) {
    let Some(sem_id) = clazzy::is_semester(clazzy) else { return };

    const DIFF: &'static str = "                              ";
    let mut classes: Vec<Vec<(String, Datey)>> = Vec::new();
    for i in 0..6 {
        classes.insert(i, Vec::new());
    }

    for class in clazzy.clazz.semesters[sem_id].classes.iter() {
        for date in class.dates.iter() {
            let day_id = date.day.num_days_from_monday() as usize;
            if let Some(weekday) = classes.get_mut(day_id) {
                weekday.push((class.name.clone(), date.clone()));
            }
        }
    }

    for i in 0..6 {
        let days = &classes[i];
        if let Some(weekday) = clazzy::into_weekday(i) {
            if !days.is_empty() {
                println!(
                    "\n{}-{}-",
                    DIFF.split_at(DIFF.len() / 2 - 2).0,
                    weekday.to_string()
                );
                for date in days.iter() {
                    println!(
                        "{}:\n{}, {}",
                        date.0,
                        date.1.from.format("%I:%M %p"),
                        date.1.to.format("%I:%M %p"),
                    );
                }
            }
        }
    }
}