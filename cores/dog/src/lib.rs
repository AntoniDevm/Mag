
#[derive(Default)]
pub struct Dog {
    pub example: u16
}

impl Dog {
    pub fn increment(&mut self) {
        self.example += 1;
    }
    pub fn value(&self) -> &u16 {
        &self.example
    }
}


