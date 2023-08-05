use crate::clazzy::Clazzy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ClazzTool {
    Zoom,
    GoogleMeets,
}

#[derive(Debug, Deserialize, Serialize)]
enum ClazzState {
    Unknown,
    Error,
    Stagnet,
    Incoming,
    JoiningClass,
    Disconnected,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub fn become_cool(clazzy: Clazzy) {
    let sleep_duration = std::time::Duration::from_secs(5);

    loop {
        std::thread::sleep(sleep_duration)
    }
}
