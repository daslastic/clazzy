use serde::{Deserialize, Serialize};
use tokio::time::{self, Duration, Interval};

use crate::clazz::Clazz;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClazzTool {
    Zoom,
    GoogleMeets,
}

#[derive(Debug, Clone)]
pub enum ClazzState {
    None,
    Error,
    Stagnet,
    Incoming,
    JoiningClass,
    Disconnected,
}

pub struct Clazzy {
    pub clazz: Clazz,
    pub duration: Duration,
    pub interval: Interval,
    pub state: ClazzState,
    pub sem_id: Option<usize>,
}

impl Clazzy {
    pub fn new(clazz: Clazz, update_interval: u64) -> Self {
        let duration = Duration::from_secs(update_interval);
        Self {
            clazz,
            duration,
            interval: time::interval(duration),
            state: ClazzState::None,
            sem_id: None,
        }
    }

    // pub fn get_sem(&self) -> &Semestery {
    //     &self.clazzy.semesters[self.clazzy.sem_id]
    // }
    //
    // pub fn get_sem_mut(&mut self) -> &mut Semestery {
    //     &mut self.clazzy.semesters[self.clazzy.sem_id]
    // }
}
