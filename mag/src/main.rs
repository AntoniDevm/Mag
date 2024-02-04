use arch::Mode;
use dog_face::DogMode;
use utils::input;
use logging::setup_logging;
use log;
fn main() {
    setup_logging(log::LevelFilter::Trace);
    log::info!("Welcome!");
    log::debug!("STARTING");
    log::warn!("WARNING");
    log::error!("SOME ERROR OCCURED");
    log::trace!("TRACING");
    loop { 
        let command = input("(mag) ").unwrap();
        println!("{}",command);
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
