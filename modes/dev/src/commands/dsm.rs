pub struct SomeCMD {
    bob: String
}

impl SomeCMD {
    pub fn sm(&mut self) {
        self.bob = String::from("AAAAAAAAAAA")
    }
    pub fn new() -> Self {
        Self { bob: String::from("Hello world")}
    }
}
