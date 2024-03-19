use arch::Mode;
use dog_face::DogMode;
use utils::input;
use logging::setup_logging;
use log;
use config::{self, Config};
use dev::{self, DevMode};

fn main() {
    setup_logging(log::LevelFilter::Trace);
    let _config = Config::new("config.yaml");
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
        let args = command.split(" ").collect::<Vec<&str>>();
        let command = if let Some(command) = args.get(0) {
            command.to_owned()
        } else {
            continue;
        };
        match command {
            "dog" => {
                let mut mode = DogMode::new();
                mode.start();
            }
            "dev" | "dv"=> {
                let mut mode = match DevMode::new(args) {
                    Ok(m) => m,
                    Err(er) => {
                        log::error!("Error entering dev mode");
                        log::debug!("Error message: {}",er);
                        continue;
                    }
                };
                mode.start(); 
            }
            "exit" | "q" | "c" | "quit"  => {
                break;
            }
            _ => {}
        }
    }
}
