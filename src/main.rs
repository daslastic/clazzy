pub mod clazzy;
pub mod domeplz;

use clazzy::{make_her, DeserializationError};

fn main() {
    let f = format!("{}/clazzy.ron", env!("CARGO_MANIFEST_DIR"));
    println!("{}", &f);
    match make_her(f) {
        Ok(x) => domeplz::become_cool(x),
        Err(e) => match e {
            DeserializationError::Io(e) => {
                println!(
                    "Wow, your either bad at cd or r stupid. Guess which one(s): {}",
                    e
                );
            }
            DeserializationError::Ron(e) => {
                println!(
                    "Failed to load hotty, maybe be more careful with your syntax: {}",
                    e
                );
            }
            DeserializationError::Idiot(e) => {
                println!("bruh how: {}", e);
            }
        },
    };
}
