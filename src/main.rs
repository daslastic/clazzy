pub mod clazzy;
pub mod data;
pub mod scheduler;

use std::sync::{Arc, Mutex};

pub use clazzy::{ClazzTool, Clazzy};
use scheduler::ProgramError;

#[tokio::main]
async fn main() {
    match start().await {
        Ok(_) => {}
        Err(e) => match e {
            ProgramError::StartLogger(e) => {
                println!("{}", e);
            }
            _ => {
                log::error!("{}", e);
            }
        },
    }
}

async fn start() -> Result<(), ProgramError> {
    simple_logger::init_with_level(log::Level::Info)?;

    let f = format!("{}/{}.ron", env!("CARGO_MANIFEST_DIR"), "conf");
    let raw_clazz = data::raw_clazz::serialize_her(f)?;

    let clazz = data::clazz::make(raw_clazz)?;

    let clazzy = Arc::new(Mutex::new(Clazzy::new(clazz)));

    scheduler::start(clazzy).await?;

    Ok(())
}