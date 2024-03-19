use super::Format;
use crate::BinFormats;
use anyhow::{self, bail, Context};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek},
    usize,
};

pub const SHF_WRITE: u32 = 0x1;
pub const SHF_ALLOC: u32 = 0x2;
pub const SHF_EXECINSTR: u32 = 0x4;
pub const SHF_MERGE: u32 = 0x10;
pub const SHF_STRINGS: u32 = 0x20;
pub const SHF_INFO_LINK: u32 = 0x40;
pub const SHF_LINK_ORDER: u32 = 0x80;
pub const SHF_OS_NONCONFORMING: u32 = 0x100;
pub const SHF_GROUP: u32 = 0x200;
pub const SHF_TLS: u32 = 0x400;
pub const SHF_MASKOS: u32 = 0x0ff00000;
pub const SHF_MASKPROC: u32 = 0xf0000000;

#[derive(Debug)]
pub struct Elf64LSB {
    pub elf_header: ElfHeader,
    pub sections: Vec<SectionHeader>,
    pub section_names: SectionHeaderTable,
    pub symbol_table: Option<Vec<Symbol>>,
}

impl Format for Elf64LSB {
    fn check(reader: &mut BufReader<&File>) -> anyhow::Result<bool> {
        reader.seek(std::io::SeekFrom::Start(0))?;
        let mut indent: [u8; 16] = [0; 16];
        reader
            .read_exact(&mut indent)
            .context(format!("Unable to read buffer, {}", reader.capacity()))?;

        // Eliminate All FIles That Don't start
        // with 0x7FELF
        if &indent[0..4] != "\x7F\x45\x4c\x46".as_bytes() {
            return Ok(false);
        }
        // Eliminate All files That are not using the 64 bit arch
        if indent[4] != 2 {
            return Ok(false);
        }

        // Allow only Little Endian
        if indent[5] != 1 {
            return Ok(false);
        }
        Ok(true)
    }
    fn parse(reader: &mut BufReader<&File>) -> anyhow::Result<BinFormats> {
        // Parsed the header
        let elf_header =
            ElfHeader::new(reader).context("Error understanding the header of the ELF File")?;

        // Parsing the SectionHeaders
        reader.seek(std::io::SeekFrom::Start(elf_header.e_shoff))?;
        let size = elf_header.e_shentsize * elf_header.e_shnum;
        let mut buff = vec![0u8; elf_header.e_shentsize as usize];
        let mut sections: Vec<SectionHeader> = Vec::new();
        for _ in (0..size).step_by(elf_header.e_shentsize as usize) {
            reader.read_exact(&mut buff)?;
            let header = SectionHeader::new(&buff)?;
            sections.push(header);
        }

        // Parsiong Section Header Table
        let index = match sections.get(elf_header.e_shstrndx as usize) {
            Some(i) => i,
            None => bail!("Tried to get section for section's name table but was not found"),
        };
        if index.sh_type != SectionHeaderType::STRTAB {
            bail!(
                "Section supposed to be of type STRTAB but {:?} was supplied",
                index.sh_type
            )
        }
        let section_names = SectionHeaderTable::parse(index, reader)?;

        let symtab_header = sections
            .iter()
            .find(|s| s.sh_type == SectionHeaderType::SYMTAB);
        let symbol_table = match symtab_header {
            Some(tab) => Some(tab),
            None => None,
        };

        let mut symbols: Option<Vec<Symbol>> = None;

        if let Some(header) = symbol_table {
            reader.seek(std::io::SeekFrom::Start(header.sh_off))?;

            let mut table: Vec<Symbol> = Vec::new();
            let mut buff: [u8; 24] = [0; 24];
            for _ in (0..header.sh_size).step_by(24) {
                reader.read_exact(&mut buff)?;
                let syb = Symbol::parse(&buff)?;
                table.push(syb)
            }
            symbols = Some(table);
        };

        Ok(BinFormats::ELF(Elf64LSB {
            elf_header,
            sections,
            section_names,
            symbol_table: symbols,
        }))
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Symbol {
    pub st_name: u32,  // size 4
    pub st_size: u64,  // size 8 => 13
    pub st_info: u8,   // size 1
    pub st_value: u64, // size 8     => 24
    pub st_shndx: u16, // size 2 => 11
    pub st_other: u8,  // size 1
}

impl Symbol {
    pub fn parse(buff: &[u8; 24]) -> anyhow::Result<Self> {
        let st_name = u32::from_le_bytes(buff[0..4].try_into().context("Unable to find st_name")?);
        let st_size = u64::from_le_bytes(buff[4..12].try_into().context("Unable to find st_size")?);
        let st_info = u8::from_le_bytes(buff[12..13].try_into().context("Unable to find st_info")?);
        let st_value =
            u64::from_le_bytes(buff[13..21].try_into().context("Unable to find st_value")?);
        let st_shndx =
            u16::from_le_bytes(buff[21..23].try_into().context("Unable to find st_shndx")?);
        let st_other =
            u8::from_le_bytes(buff[23..24].try_into().context("Unable to find st_other")?);

        Ok(Self {
            st_name,
            st_other,
            st_shndx,
            st_value,
            st_info,
            st_size,
        })
    }
}

#[derive(Debug)]
pub struct SectionHeaderTable {
    pub names: HashMap<u16, String>,
}

impl SectionHeaderTable {
    pub fn parse(section: &SectionHeader, reader: &mut BufReader<&File>) -> anyhow::Result<Self> {
        reader.seek(std::io::SeekFrom::Start(section.sh_off))?;
        let mut buff = vec![0u8; section.sh_size as usize];
        reader
            .read_exact(&mut buff)
            .context("Error reading the Section Header Table")?;

        let mut names: HashMap<u16, String> = HashMap::new();
        let mut name = String::new();
        let mut index: u16 = 0;
        for byte in buff {
            if byte != 0x0 {
                let c = byte as char;
                name.push(c);
            } else {
                names.insert(index as u16, name.clone()); // Section names are not that big. You
                                                          // can fix it in the Future ;)
                index += 1;
                name.clear();
            };
        }
        Ok(Self { names })
    }
}

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: SectionHeaderType,
    pub sh_flags: SectionHeaderFlags,
    pub sh_addr: u64,
    pub sh_off: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl SectionHeader {
    pub fn new(buff: &[u8]) -> anyhow::Result<Self> {
        let sh_name = u32::from_le_bytes(
            buff[0..4]
                .try_into()
                .context("Unable to find the name member in the section header")?,
        );
        let sh_type = match u32::from_le_bytes(
            buff[4..8]
                .try_into()
                .context("Unable to find the type member in the section header")?,
        ) {
            1 => SectionHeaderType::PROGBITS,
            2 => SectionHeaderType::SYMTAB,
            3 => SectionHeaderType::STRTAB,
            4 => SectionHeaderType::RELA,
            5 => SectionHeaderType::HASH,
            6 => SectionHeaderType::DYNAMIC,
            7 => SectionHeaderType::NOTE,
            8 => SectionHeaderType::NOBITS,
            9 => SectionHeaderType::REL,
            10 => SectionHeaderType::SHLIB,
            11 => SectionHeaderType::DYNSYM,
            0x70000000 => SectionHeaderType::LOPROC,
            0x7fffffff => SectionHeaderType::HIPROC,
            0x80000000 => SectionHeaderType::LOUSER,
            0xffffffff => SectionHeaderType::HIUSER,
            _ => SectionHeaderType::NULL,
        };

        let flags = u32::from_le_bytes(
            buff[8..12]
                .try_into()
                .context("Unable to aprse the Flags member")?,
        );
        // Parsing the flags
        let mut sh_flags = SectionHeaderFlags::default();
        let _ = (flags & SHF_WRITE) != 0 && (sh_flags.write = true) == ();
        let _ = (flags & SHF_ALLOC) != 0 && (sh_flags.alloc = true) == ();
        let _ = (flags & SHF_EXECINSTR) != 0 && (sh_flags.execinstr = true) == ();
        let _ = (flags & SHF_STRINGS) != 0 && (sh_flags.strings = true) == ();
        let _ = (flags & SHF_INFO_LINK) != 0 && (sh_flags.info_link = true) == ();
        let _ = (flags & SHF_LINK_ORDER) != 0 && (sh_flags.link_order = true) == ();
        let _ = (flags & SHF_OS_NONCONFORMING) != 0 && (sh_flags.os_nonconforming = true) == ();
        let _ = (flags & SHF_GROUP) != 0 && (sh_flags.group = true) == ();
        let _ = (flags & SHF_TLS) != 0 && (sh_flags.tls = true) == ();
        let _ = (flags & SHF_MASKOS) != 0 && (sh_flags.maskos = true) == ();
        let _ = (flags & SHF_MASKPROC) != 0 && (sh_flags.maskproc = true) == ();
        let _ = (flags & SHF_MASKPROC) != 0 && (sh_flags.maskproc = true) == ();
        let _ = (flags & SHF_MERGE) != 0 && (sh_flags.merge = true) == ();

        let sh_addr =
            u64::from_le_bytes(buff[16..24].try_into().context("Unable to parse sh_addr")?);
        let sh_off = u64::from_le_bytes(
            buff[24..32]
                .try_into()
                .context("Unable to parse sh_offset")?,
        );
        let sh_size =
            u64::from_le_bytes(buff[32..40].try_into().context("Unable to parse sh_size")?);
        let sh_link =
            u32::from_le_bytes(buff[40..44].try_into().context("Unable to parse sh_link")?);
        let sh_info =
            u32::from_le_bytes(buff[44..48].try_into().context("Unable to parse sh_info")?);
        let sh_addralign =
            u64::from_le_bytes(buff[48..56].try_into().context("Unable to parse sh_addr")?);
        let sh_entsize = u64::from_le_bytes(
            buff[56..64]
                .try_into()
                .context("Unable to parse sh_entsize")?,
        );

        Ok(Self {
            sh_addralign,
            sh_name,
            sh_flags,
            sh_link,
            sh_size,
            sh_addr,
            sh_off,
            sh_type,
            sh_info,
            sh_entsize,
        })
    }
}
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum SectionHeaderType {
    NULL,
    PROGBITS,
    SYMTAB,
    STRTAB,
    RELA,
    HASH,
    DYNAMIC,
    NOTE,
    NOBITS,
    REL,
    SHLIB,
    DYNSYM,
    LOPROC,
    HIPROC,
    LOUSER,
    HIUSER,
}
#[derive(Debug, Default, PartialEq)]
#[allow(unused)]
pub struct SectionHeaderFlags {
    pub write: bool,
    pub alloc: bool,
    pub execinstr: bool,
    pub merge: bool,
    pub strings: bool,
    pub info_link: bool,
    pub link_order: bool,
    pub os_nonconforming: bool,
    pub group: bool,
    pub tls: bool,
    pub maskos: bool,
    pub maskproc: bool,
}

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct ElfHeader {
    pub e_type: ElfType,
    pub e_machine: ElfMachine,
    pub e_version: ElfVersion,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ElfHeader {
    pub fn new(reader: &mut BufReader<&File>) -> anyhow::Result<Self> {
        reader.seek(std::io::SeekFrom::Start(16))?;
        let mut elf_header: [u8; 48] = [0; 48];
        reader.read_exact(&mut elf_header)?;
        let e_type = match elf_header[0..=1] {
            [0x1, 0x00] => ElfType::REL,
            [0x2, 0x00] => ElfType::EXEC,
            [0x3, 0x00] => ElfType::DYN,
            [0x4, 0x00] => ElfType::CORE,
            [0xff, 0x00] => ElfType::LOPROC,
            [0xff, 0xff] => ElfType::HIPROC,
            _ => ElfType::NONE,
        };
        let e_machine = match elf_header[2..=3] {
            [0x3E, 0x00] => ElfMachine::EmX86_64,
            _ => ElfMachine::NONE,
        };
        let e_version = match elf_header[4..=7] {
            [0x01, 0x00, 0x00, 0x00] => ElfVersion::Current,
            _ => ElfVersion::None,
        };
        let e_entry = u64::from_le_bytes(
            elf_header[0x8..=0xF]
                .try_into()
                .context("Error converting e_entry")?,
        );
        let e_phoff = u64::from_le_bytes(
            elf_header[0x10..=0x17]
                .try_into()
                .context("Error converting e_phoff")?,
        );
        let e_shoff = u64::from_le_bytes(
            elf_header[0x18..=0x1F]
                .try_into()
                .context("Error converting e_shoff")?,
        );
        let e_flags = u32::from_le_bytes(
            elf_header[0x20..=0x23]
                .try_into()
                .context("Error converting e_flags")?,
        );
        let e_ehsize = u16::from_le_bytes(
            elf_header[0x24..=0x25]
                .try_into()
                .context("Error converting e_ehsize")?,
        );
        let e_phentsize = u16::from_le_bytes(
            elf_header[0x26..=0x27]
                .try_into()
                .context("Error converting e_phentsize")?,
        );
        let e_phnum = u16::from_le_bytes(
            elf_header[0x28..=0x29]
                .try_into()
                .context("Error converting e_phnum")?,
        );
        let e_shentsize = u16::from_le_bytes(
            elf_header[0x2A..=0x2B]
                .try_into()
                .context("Error converting e_shentsize")?,
        );
        let e_shnum = u16::from_le_bytes(
            elf_header[0x2C..=0x2D]
                .try_into()
                .context("Error converting e_shnum")?,
        );
        let e_shstrndx = u16::from_le_bytes(
            elf_header[0x2E..=0x2F]
                .try_into()
                .context("Error converting e_shstrndx")?,
        );
        Ok(Self {
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElfType {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    LOPROC,
    HIPROC,
}

#[derive(Debug, PartialEq, Eq)]
/// Not all machines supported. Feel free to implement
pub enum ElfMachine {
    NONE,
    EmX86_64, //  62	AMD x86-64 architecture
}
#[derive(Debug, PartialEq, Eq)]
pub enum ElfVersion {
    Current,
    None,
}
