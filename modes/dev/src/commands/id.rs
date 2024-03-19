use arch::Command;
use binid::BinFormats;
use std::{cell::RefCell, rc::Rc};

pub struct Identify {
    core: Rc<RefCell<binid::BinID>>,
    // path: &'a str
}

impl Identify {
    pub fn new(core: &Rc<RefCell<binid::BinID>>) -> Identify {
        Identify {
            core: Rc::clone(core),
            // path
        }
    }
}

impl Command for Identify {
    fn run(&mut self, args: Vec<&str>) {
        let mut core = match self.core.try_borrow_mut() {
            Ok(c) => c,
            Err(er) => {
                log::error!("Error borring binid core");
                log::debug!("Error message: {:#?}", er);
                return;
            }
        };
        let formats = match core.analyze() {
            Ok(format) => format,
            Err(er) => {
                log::error!("Error analyzing file");
                log::debug!("Error message: {:#?}", er);
                return;
            }
        };
        log::debug!("Running ID");
        match formats {
            BinFormats::ELF(format) => {
                // log::info!("Indentified {:#X?}",format)
                let filter = if let Some(filter) = args.get(1) {
                    filter
                } else {
                    "all"
                };
                if filter.contains("h") || filter == "all" {
                    log::info!("Elf Header");
                    log::info!("Type: {:?}", format.elf_header.e_type);
                    log::info!("Entry: {}", format.elf_header.e_entry);
                    log::info!("Offset: {}", format.elf_header.e_shoff);
                    log::info!("Program Header Offset: {}", format.elf_header.e_phoff);
                    log::info!("Section Header Size: {}", format.elf_header.e_shentsize);
                    log::info!("Program Header Size: {}", format.elf_header.e_phentsize);
                    log::info!("Section Header Name idex: {}", format.elf_header.e_shstrndx);
                    log::info!("Version: {:?}", format.elf_header.e_version);
                    log::info!("Machine: {:?}", format.elf_header.e_machine);
                    log::info!("Version: {:?}", format.elf_header.e_flags);
                }

                if filter.contains("sn") || filter == "all" {
                    log::info!("Section Indexes & names ");
                    for (index, name) in format.section_names.names {
                        log::info!("{:>3X} => {}", index, name);
                    }
                }

                if filter.contains("sc") || filter == "all" {
                    log::info!("Sections");
                    for section in format.sections {
                        log::info!("Name: {}", section.sh_name);
                        log::info!("Offset: {}", section.sh_off);
                        log::info!("Type: {:?}", section.sh_type);
                        log::info!("Addr: {}", section.sh_addr);
                        log::info!("Size: {}", section.sh_size);
                        log::info!("Flags: {:?}", section.sh_flags);
                        log::info!("Link: {:?}", section.sh_link);
                        log::info!("Addr Align: {:?}", section.sh_addralign);
                        log::info!("EntSize: {:?}", section.sh_entsize);
                        log::info!("=========================== ")
                    }
                }

                if filter.contains("sb") || filter == "all" {
                    match format.symbol_table {
                        Some(tb) => {
                            for symb in tb {
                                log::info!(
                                    "NM: {:<9X} SZ: {:<12X} INF: {:<4X} VL: {:<10X} IDX: {:X} OTH: {:X}",
                                    symb.st_name,
                                    symb.st_size,
                                    symb.st_info,
                                    symb.st_value,
                                    symb.st_shndx,
                                    symb.st_other
                                )
                            }
                        }
                        None => {
                            log::error!("No Symbol table found")
                        }
                    };
                }
                log::debug!("{:?}",args);
            }
            #[allow(unreachable_patterns)]
            _ => (),
        };
    }
}
