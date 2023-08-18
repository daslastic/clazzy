pub mod clazzy;
pub mod data;
pub mod error;
pub mod scheduler;

use std::sync::{Arc, Mutex};

pub use clazzy::{ClazzTool, Clazzy};
pub use error::ProgramError;

fn main() {
    match start() {
        Err(e) => match e {
            ProgramError::SetLogger(e) => {
                println!("{}", e);
            }
            _ => {
                log::error!("{}", e);
            }
        },
        _ => {}
    }
}

fn start() -> Result<(), ProgramError> {
    simple_logger::init_with_level(log::Level::Info)?;

    let f = format!("{}/{}.ron", env!("CARGO_MANIFEST_DIR"), "conf");
    let raw_clazz = data::raw_clazz::serialize_her(f)?;
    let clazz = data::clazz::make(raw_clazz)?;
    let clazzy = Arc::new(Mutex::new(Clazzy::new(clazz)));

    scheduler::start(clazzy)?;

    Ok(())
}