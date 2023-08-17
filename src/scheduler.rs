use std::{
    error::Error,
    fmt::{self, Pointer},
    sync::{Arc, Mutex},
};

use crate::{
    data::{clazz::ClazzError, raw_clazz::DeserializationError},
    Clazzy,
};
use chrono::Timelike;
use log::SetLoggerError;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn start(clazzy_ref: Arc<Mutex<Clazzy>>) -> Result<(), ProgramError> {
    let scheduler = JobScheduler::new().await?;

    let mut dates: Vec<(String, (usize, usize, usize))> = Vec::new();

    {
        let clazzy = clazzy_ref.try_lock().unwrap();
        let sem_id = clazzy.sem_id.unwrap();
        for (i, class) in clazzy.clazz.semesters[sem_id].classes.iter().enumerate() {
            for (o, date) in class.dates.iter().enumerate() {
                let pos = (sem_id, i, o);

                let mut min = String::new();
                min.push_str(&date.from.minute().to_string());
                min.push('-');
                min.push_str(&date.to.minute().to_string());

                let mut hour = String::new();
                hour.push_str(&date.from.hour().to_string());
                hour.push('-');
                hour.push_str(&date.to.hour().to_string());

                let str = format!("* {} {} * * {}", min, hour, date.day.to_string());
                log::info!("{}", str);

                dates.push((str, pos));
            }

            log::info!("Class '{}' is setup!", class.name);
        }
    }

    for date in dates {
        let clazzy_ref = clazzy_ref.clone();
        scheduler
            .add(Job::new(date.0.as_str(), move |_uuid, _l| {
                let pos = date.1;
                let mut clazzy = clazzy_ref.try_lock().unwrap();

                let class = &mut clazzy.clazz.semesters[pos.0].classes[pos.1];
                let date = &mut class.dates[pos.2];

                log::info!("Class '{}' active", class.name);
            })?)
            .await?;
    }

    scheduler.start().await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
    }
}

#[derive(Debug)]
pub enum ProgramError {
    JobSchedulerError(JobSchedulerError),
    ClazzError(ClazzError),
    DeserializationError(DeserializationError),
    StartLogger(SetLoggerError),
}

impl Error for ProgramError {}
impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgramError::JobSchedulerError(e) => write!(f, "Job scheduler error: {}", e),
            ProgramError::ClazzError(e) => write!(f, "Clazz error: {}", e),
            ProgramError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
            ProgramError::StartLogger(e) => write!(f, "Logger error: {}", e),
        }
    }
}

impl From<JobSchedulerError> for ProgramError {
    fn from(e: JobSchedulerError) -> Self {
        return ProgramError::JobSchedulerError(e);
    }
}

impl From<ClazzError> for ProgramError {
    fn from(e: ClazzError) -> Self {
        return ProgramError::ClazzError(e);
    }
}

impl From<DeserializationError> for ProgramError {
    fn from(e: DeserializationError) -> Self {
        return ProgramError::DeserializationError(e);
    }
}

impl From<SetLoggerError> for ProgramError {
    fn from(e: SetLoggerError) -> Self {
        return ProgramError::StartLogger(e);
    }
}

