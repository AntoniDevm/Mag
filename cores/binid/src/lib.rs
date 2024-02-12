mod formats;
use formats::{Format, ElfExecuable};
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufReader};

pub struct BinID {
    file: File,
}

impl BinID {
    pub fn new(path: &str) -> Result<BinID, io::Error> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(er) => {
                log::error!("Error reading file");
                log::debug!("Error message: {}", er);
                return Err(io::Error::new(io::ErrorKind::NotFound, er));
            }
        };
        Ok(BinID { file })
    }
    pub fn analyze(&mut self) -> Result<BinFormats, BinIDError> {
        let mut reader = BufReader::new(&self.file);
        if ElfExecuable::check(&mut reader) {
            return Ok(ElfExecuable::parse(&mut reader))
        };
        Ok(BinFormats::UNKNOWN)
    }
}
pub enum BinIDError {
    FormatNotRegognized,
    BufferUnreadable { er: String },
}
impl Display for BinIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinIDError::BufferUnreadable {er} => {
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


pub enum BinFormats {
    ELF(ElfExecuable),
    PE,
    UNKNOWN,
}
