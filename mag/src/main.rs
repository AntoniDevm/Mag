use arch::Mode;
use dog_face::DogMode;
use utils::input;
use logging::setup_logging;
use log;
fn main() {
    setup_logging(log::LevelFilter::Trace);
    log::info!("Welcome!");
    loop { 
        let command = match input("(mag) ") {
            Ok(s) => s,
            Err(er) => {
                log::error!("An error occured while getting user input");
                log::debug!("Error message: {}",er);
                return;

            }
        };

        match command.as_str() {
            "dog" => {
                let mut mode = DogMode::new();
                mode.start();
            }
            "exit" | "q" | "c" | "quit"  => {
                break;
            }
            _ => {}
        }
    }
}
