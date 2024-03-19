pub mod formats;
use anyhow::bail;
use formats::{Format, Elf64LSB};
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

pub struct BinID {
    file: File,
}

impl BinID {
    pub fn new(path: &str) -> anyhow::Result<BinID> {
        let file = File::open(path)?;
        Ok(BinID { file })
    }
    pub fn analyze(&mut self) -> anyhow::Result<BinFormats> {
        let mut reader = BufReader::new(&self.file);
        
        if Elf64LSB::check(&mut reader)? {
            return Ok(Elf64LSB::parse(&mut reader)?)
        };
        bail!("Unknows Format")
    }
    pub fn set(&mut self, path: &str) -> anyhow::Result<()> {
        
        self.file = File::open(path)?;
        Ok(())
    }
}

pub enum BinFormats {
    ELF(Elf64LSB),
    PE,
    UNKNOWN,
}

pub enum BinIDError {
    FormatNotRegognized,
    BufferError { er: String },
}

impl Display for BinIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinIDError::BufferError {er} => {
                write!(f, "Can not read buffer. {}", er)
            }
            BinIDError::FormatNotRegognized => {
                write!(f, "Format was not regognized")
            }
            #[allow(unreachable_patterns)]
            _ => {
                write!(f, "Unknown Error")
            }
        }
    }
}



