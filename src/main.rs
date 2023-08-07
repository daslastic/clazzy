pub mod clazzy;
pub mod data;
pub mod scheduler;

use std::sync::{Arc, Mutex};

pub use clazzy::{ClazzTool, Clazzy};
use scheduler::ProgramError;

#[tokio::main]
async fn main() -> Result<(), ProgramError<'static>> {
    simple_logger::init_with_level(log::Level::Info).expect("Failed to start logger.");

    let f = format!("{}/{}.ron", env!("CARGO_MANIFEST_DIR"), "conf");
    match data::raw_clazz::serialize_her(f) {
        Ok(raw_clazz) => match data::clazz::make(raw_clazz) {
            Ok(clazz) => {
                let clazzy = Arc::new(Mutex::new(Clazzy::new(clazz)));
                scheduler::start(clazzy).await?;
            }
            Err(e) => log::error!("{}", e),
        },
        Err(e) => log::error!("{}", e),
    };

    Ok(())
}
