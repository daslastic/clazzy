pub mod clazzy;
pub mod data;
pub mod manclazz;

use std::error::Error;

pub use clazzy::{ClazzTool, Clazzy};

use data::raw_clazz::{serialize_her, DeserializationError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).expect("Failed to start logger.");

    let f = format!("{}/{}.ron", env!("CARGO_MANIFEST_DIR"), "conf");
    match serialize_her(f) {
        Ok(raw_clazz) => match data::clazz::make_clazz(raw_clazz) {
            Ok(clazzy) => {
                let mut context = Clazzy::new(clazzy, 10);
                loop {
                    context.interval.tick().await;
                    run(&mut context).await;
                }
            }
            Err(e) => log::error!("{}", e),
        },
        Err(e) => match e {
            DeserializationError::Io(e) => {
                log::error!(
                    "Wow, your either bad at cd or r stupid. Guess which one(s): {}",
                    e
                );
            }
            DeserializationError::Ron(e) => {
                log::error!(
                    "Failed to load hotty, maybe be more careful with your syntax: {}",
                    e
                );
            }
            DeserializationError::Idiot(e) => {
                log::error!("bruh how: {}", e);
            }
        },
    };

    Ok(())
}

async fn run(clazzy: &mut Clazzy) {
    // manclazz::check_time(clazzy);
}
