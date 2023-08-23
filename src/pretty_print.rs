use crate::{
    clazzy::{self},
    data::clazz::Datey,
    Clazzy,
};

const TIME_FORMAT: &'static str = "%I:%M %p";
const DIFF: &'static str = "                              ";

pub fn sexy(clazzy: &mut Clazzy) {
    let Some(sem_id) = clazzy.is_semester() else {
        log::info!("No semester applies to today :)");
        return
    };

    let mut classes: Vec<Vec<(String, Datey)>> = Vec::new();
    for i in 0..6 {
        classes.insert(i, Vec::new());
    }

    for class in clazzy.clazz.semesters[sem_id].classes.iter() {
        for date in class.dates.iter() {
            if let Some(days) = classes.get_mut(date.day.num_days_from_monday() as usize) {
                days.push((class.name.clone(), date.clone()));
            }
        }
    }

    for i in 0..6 {
        let days = &mut classes[i];
        days.sort_by_key(|(_, datey)| datey.from);
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
                        date.1.from.format(TIME_FORMAT),
                        date.1.to.format(TIME_FORMAT),
                    );
                }
            }
        }
    }

    println!(
        "\n({}/{})",
        clazzy.clazz.semesters[sem_id].from, clazzy.clazz.semesters[sem_id].to,
    );
}