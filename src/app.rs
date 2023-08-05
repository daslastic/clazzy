use serde::{Deserialize, Serialize};
use std::{thread::sleep, time::Duration};

use crate::clazzy::Clazzy;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClazzTool {
    Zoom,
    GoogleMeets,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum ClazzState {
    Unknown,
    Error,
    Stagnet,
    Incoming,
    JoiningClass,
    Disconnected,
}

pub fn init(clazzy: Clazzy) {
    let sleep_duration = Duration::from_secs(5);
    loop {
        println!("wow");
        sleep(sleep_duration)
    }
}
