mod elf64;
use anyhow;
use std::{fs::File, io::BufReader};

pub use elf64::*;//{Elf64LSB, ElfHeader, ElfMachine, ElfType, ElfVersion, Section};

use crate::BinFormats;

pub trait Format {
    /// This Function will check if the header matches the format
    fn check(reader: &mut BufReader<&File>) -> anyhow::Result<bool>;
    fn parse(reader: &mut BufReader<&File>) -> anyhow::Result<BinFormats>;
}
