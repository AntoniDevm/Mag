use std::{rc::Rc,cell::RefCell};
use dog::Dog;

pub struct Bark {
    core: Rc<RefCell<Dog>>
}

impl arch::Command for Bark {
    fn run(&mut self, _args: Vec<&str>) {
        println!("Woff Woff");
        let mut core = self.core.borrow_mut();
        core.increment();
    }
}

impl Bark {
    pub fn new(core: Rc<RefCell<Dog>>) -> Bark {
        Bark {
            core
        } 
    }
}
