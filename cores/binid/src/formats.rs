mod elf;
use std::{io::BufReader, fs::File};

pub use elf::ElfExecuable;

use crate::BinFormats;


pub trait Format {
    /// This Function will check if the header matches the format
    fn check(reader: &mut BufReader<&File>) -> bool;
    fn parse(reader: &mut BufReader<&File>) -> BinFormats;
}
