pub mod clazzy;
pub mod data;
pub mod error;
pub mod pretty_print;
pub mod scheduler;

pub use clazzy::{ClazzTool, Clazzy, DatePos};
pub use error::ProgramError;

use std::sync::{Arc, Mutex};

pub const APP_NAME: &'static str = "clazzy";
pub const CONFIG: &'static str = "conf";

fn main() {
    if let Err(e) = start() {
        match e {
            ProgramError::SetLogger(e) => {
                println!("{}", e);
            }
            _ => {
                log::error!("{}", e);
            }
        }
    }
}

fn start() -> Result<(), ProgramError> {
    simple_logger::init_with_level(log::Level::Info)?;

    let raw_clazz: data::raw_clazz::RawClazz = confy::load(APP_NAME, CONFIG)?;
    let path = confy::get_configuration_file_path(APP_NAME, CONFIG)?;
    let clazz = data::clazz::make(raw_clazz)?;
    let clazzy = Arc::new(Mutex::new(Clazzy::new(clazz)));

    log::info!("{:?}", path);
    scheduler::start(clazzy)?;

    Ok(())
}