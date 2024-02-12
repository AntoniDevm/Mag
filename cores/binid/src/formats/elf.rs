use std::{io::{BufReader, Read}, fs::File};
use crate::BinFormats;

use super::Format;


pub struct ElfExecuable;

impl Format for ElfExecuable {
    fn check(reader: &mut BufReader<&File>) -> bool {
        let mut signature: [u8;4] = [0;4];
        match reader.read_exact(&mut signature) {
            Ok(_) => {}
            Err(er) => {
                log::error!("Error reading file");
                log::debug!("Error message: {}", er);
                return false;
            }
        };
        if signature != "\x7F\x45\x4c\x46".as_bytes() { 
            // Eliminate All FIles That Don't start
            // with ELF
            return false;   
        }
        true
    }
    fn parse(reader: &mut BufReader<&File>) -> BinFormats {
        BinFormats::ELF(ElfExecuable)
   }
}
