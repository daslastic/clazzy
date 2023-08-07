use std::{
    error::Error,
    fmt::Display,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use crate::{
    data::{clazz::ClazzError, raw_clazz::DeserializationError},
    Clazzy,
};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

// prototype
pub async fn start(clazzy_ref: Arc<Mutex<Clazzy>>) -> Result<(), ProgramError<'static>> {
    let scheduler = JobScheduler::new().await?;

    scheduler.start().await?;
    let clazzy = clazzy_ref.lock().unwrap();

    for (i, class) in clazzy.clazz.semesters[clazzy.sem_id.unwrap()]
        .classes
        .iter()
        .enumerate()
    {
        for (o, date) in class.dates.iter().enumerate() {
            let min = "*";
            let hour = "*";
            let day = "*";
            let month = "*";
            let day_of_week = date.day.to_string();
            let str = format!("* {} {} {} {} {}", min, hour, day, month, day_of_week);

            let to_clone = Arc::clone(&clazzy_ref);
            scheduler
                .add(Job::new(str.as_str(), move |_uuid, _l| {
                    process_class(to_clone.clone(), (i, o));
                })?)
                .await?;
        }
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
    }
}

pub fn process_class(
    clazzy_ref: Arc<Mutex<Clazzy>>,
    pos: (usize, usize),
) -> Result<(), ProgramError<'static>> {
    let clazzy = clazzy_ref.lock().unwrap();
    Ok(())
}

#[derive(Debug)]
pub enum ProgramError<'a> {
    MutexError(PoisonError<MutexGuard<'a, Clazzy>>),
    JobSchedulerError(JobSchedulerError),
    ClazzError(ClazzError),
    DeserializationError(DeserializationError),
}

impl<'a> Error for ProgramError<'a> {}

impl<'a> Display for ProgramError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            e => {
                write!(f, "{}", e)
            }
        }
    }
}

impl<'a> From<JobSchedulerError> for ProgramError<'a> {
    fn from(e: JobSchedulerError) -> Self {
        return ProgramError::JobSchedulerError(e);
    }
}

impl<'a> From<PoisonError<MutexGuard<'a, Clazzy>>> for ProgramError<'a> {
    fn from(e: PoisonError<MutexGuard<'a, Clazzy>>) -> Self {
        return ProgramError::MutexError(e);
    }
}

impl<'a> From<ClazzError> for ProgramError<'a> {
    fn from(e: ClazzError) -> Self {
        return ProgramError::ClazzError(e);
    }
}

impl<'a> From<DeserializationError> for ProgramError<'a> {
    fn from(e: DeserializationError) -> Self {
        return ProgramError::DeserializationError(e);
    }
}
