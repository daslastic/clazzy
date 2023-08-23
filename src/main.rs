pub mod clazzy;
pub mod data;
pub mod error;
pub mod notification;
pub mod pretty_print;
pub mod scheduler;

pub use clazzy::{ClazzTool, Clazzy, DatePos};
pub use error::ProgramError;

use std::sync::{Arc, Mutex};

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

    let raw_clazz: data::raw_clazz::RawClazz = confy::load("clazzy", "conf")?;
    let clazz = data::clazz::make(raw_clazz)?;
    let clazzy = Arc::new(Mutex::new(Clazzy::new(clazz)));

    scheduler::start(clazzy)?;

    Ok(())
}
