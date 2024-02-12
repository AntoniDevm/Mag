use std::{rc::Rc, cell::RefCell};
use binid::BinFormats;
use arch::Command;



pub struct Identify<'a> { 
    core: Rc<RefCell<binid::BinID>>,
    path: &'a str
}

impl<'a> Identify<'a> {
    pub fn new(core: &Rc<RefCell<binid::BinID>>,path: &'a str) -> Identify<'a> {
        Identify {
            core: Rc::clone(core),
            path
        }
    }
}

impl<'a> Command for Identify<'_> {
    fn run(&mut self) {
        let mut core = match self.core.try_borrow_mut() {
            Ok(c) => c,
            Err(er) => {
                log::error!("Error borring binid core");
                log::debug!("Error message: {}",er);
                return;
            }
        };
        let formats = match core.analyze() {
            Ok(format) => {
                format
            },
            Err(er) => {
                log::error!("Error analyzing file");
                log::debug!("Error message: {}",er);
                return;
            }
        };
        log::debug!("Running ID");
        match formats {
            BinFormats::ELF(format) => {
                println!("IT'S AN ELF")
            }
            #[allow(unreachable_patterns)]
            _ => ()
        };
        
    }
}



