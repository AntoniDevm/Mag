use std::fmt::Display;

pub trait Mode {
    fn start(&mut self);
    fn end(&mut self) {}
}

pub trait Command {
    fn run(&mut self, args: Vec<&str> );
    fn help() -> &'static str {
        "The commnad is not documented yet."
    }
}

#[derive(Debug)]
pub enum ModeError {
    ErrorCreatingCore,
    CoreError,
}

impl Display for ModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ModeError {}

