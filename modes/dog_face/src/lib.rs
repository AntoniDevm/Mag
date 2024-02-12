mod commands;
use std::{rc::Rc, cell::RefCell};
use arch::{Command, Mode};
use utils::input;
use commands::{Bark, Count};
use dog::Dog;


pub struct DogMode {
    bark: Bark,
    counter: Count,
}

impl DogMode {
    pub fn new() -> DogMode {
        let core = Rc::new(RefCell::new(Dog::default()));
        let counter = Count::new(Rc::clone(&core));
        let bark = Bark::new(Rc::clone(&core));
        DogMode {
            bark,
            counter
        }
    }
}

impl Mode for DogMode {
    fn start(&mut self) {
        loop {
            let command = match input("(dog) ") {
                Ok(s) => s,
                Err(er) => {
                    log::error!("An error occured while getting user input");
                    log::debug!("Error message: {}", er);
                    return;
                }
            };
            match command.as_str() {
                "bark" => {
                    self.bark.run();
                },
                "count" => {
                    self.counter.run();
                }
                "exit" | "c" | "q" | "quit" => {
                    break;
                }
                _ => {}
            }
        }
    }
}
