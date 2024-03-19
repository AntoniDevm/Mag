use std::{rc::Rc,cell::RefCell};
use dog::Dog;

pub struct Count {
    core: Rc<RefCell<Dog>>
}

impl arch::Command for Count {
    fn run(&mut self, _args: Vec<&str>) {
        let core = self.core.borrow();
        println!("I've barked {} times", core.value())
    }
}

impl Count {
    pub fn new(core: Rc<RefCell<Dog>>) -> Count {
        Count {
            core
        } 
    }
}

