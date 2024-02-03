pub trait Mode {
    fn start(&mut self);
    fn end(&mut self) {}
}

pub trait Command {
    fn run(&mut self);
    fn help() -> &'static str {
        "The commnad is not documented yet."
    }
}
