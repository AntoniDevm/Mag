use std::{cell::RefCell, io};
use std::rc::Rc;

use arch::{Command, Mode};
use binid::BinID;
use commands::Identify;
use utils::input;
mod commands;

pub struct DevMode<'a> {
    #[allow(dead_code)]
    args: Vec<&'a str>,
    id: Identify<'a>
}

impl<'a> Mode for DevMode<'a> {
    fn start(&mut self) {
        loop {
            let command = match input("(dev) ") {
                Ok(c) => c,
                Err(er) => {
                    log::error!("Error reading user input");
                    log::debug!("Error message: {}", er);
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
                "id" => {
                    self.id.run();
                }
                "exit" | "quit" | "q" => {
                    break;
                }

                _ => {}
            }
        }
    }
}

impl<'a> DevMode<'a> {
    pub fn new(args: Vec<&str>) -> Result<DevMode,io::Error> {
        let file = if let Some(file) = args.get(1) {
            file
        } else {
            log::error!("Not enough arguments. Supply file name");
            return Err(io::Error::new(io::ErrorKind::InvalidData, "More arduments required"));
        };
        let binid = match BinID::new(file) {
            Ok(b) => b,
            Err(er) => {
                log::error!("Error creating BinID core");
                log::debug!("Error message: {}",er);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, er));
            }
        };
        let core = Rc::new(RefCell::new(binid));
        let id = Identify::new(&core, file);
        Ok(DevMode { args, id })
    }
}
