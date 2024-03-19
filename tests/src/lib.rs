#[cfg(test)]
mod elfheader {
    use std::{fs::File, io::BufReader};

    use binid::formats::{ElfHeader, ElfMachine, ElfType, ElfVersion};
    #[test]
    fn dynamic_64() {
        let f = File::open("samples/d64").unwrap();
        let mut reader = BufReader::new(&f);
        let hdr = ElfHeader::new(&mut reader).unwrap();
        assert_eq!(hdr.e_type, ElfType::DYN);
        assert_eq!(hdr.e_machine, ElfMachine::EmX86_64);
        assert_eq!(hdr.e_version, ElfVersion::Current);
        assert_eq!(hdr.e_entry, 4224);
        assert_eq!(hdr.e_phoff, 64);
        assert_eq!(hdr.e_shoff, 14024);
        assert_eq!(hdr.e_flags, 0);
        assert_eq!(hdr.e_ehsize, 64);
        assert_eq!(hdr.e_phentsize, 56);
        assert_eq!(hdr.e_phnum, 13);
        assert_eq!(hdr.e_shentsize, 64);
        assert_eq!(hdr.e_shnum, 31);
        assert_eq!(hdr.e_shstrndx, 30)
    }
    #[test]
    fn static_64() {
        let f = File::open("samples/s64").unwrap();
        let mut reader = BufReader::new(&f);
        let hdr = ElfHeader::new(&mut reader).unwrap();

        assert_eq!(hdr.e_type, ElfType::EXEC);
        assert_eq!(hdr.e_machine, ElfMachine::EmX86_64);
        assert_eq!(hdr.e_version, ElfVersion::Current);
        assert_eq!(hdr.e_entry, 4199968);
        assert_eq!(hdr.e_phoff, 64);
        assert_eq!(hdr.e_shoff, 898296);
        assert_eq!(hdr.e_flags, 0);
        assert_eq!(hdr.e_ehsize, 64);
        assert_eq!(hdr.e_phentsize, 56);
        assert_eq!(hdr.e_phnum, 10);
        assert_eq!(hdr.e_shentsize, 64);
        assert_eq!(hdr.e_shnum, 32);
        assert_eq!(hdr.e_shstrndx, 31);
    }
    #[test]
    fn relocatable_64() {
        let f = File::open("samples/r64").unwrap();
        let mut reader = BufReader::new(&f);
        let hdr = ElfHeader::new(&mut reader).unwrap();
        assert_eq!(hdr.e_type, ElfType::REL);
        assert_eq!(hdr.e_machine, ElfMachine::EmX86_64);
        assert_eq!(hdr.e_version, ElfVersion::Current);
        assert_eq!(hdr.e_entry, 0);
        assert_eq!(hdr.e_phoff, 0);
        assert_eq!(hdr.e_shoff, 944);
        assert_eq!(hdr.e_flags, 0);
        assert_eq!(hdr.e_ehsize, 64);
        assert_eq!(hdr.e_phentsize, 0);
        assert_eq!(hdr.e_phnum, 0);
        assert_eq!(hdr.e_shentsize, 64);
        assert_eq!(hdr.e_shnum, 14);
        assert_eq!(hdr.e_shstrndx, 13);
    }
}

#[cfg(test)]
mod sectionheader {
    use binid::formats::{ElfHeader, SectionHeader};
    use object::{Object, ObjectSection};
    use std::{
        fs::{self, File},
        io::{BufReader, Read, Seek},
    };
    #[test]
    fn dynamic_64() {
        let path = "samples/d64";
        let f = File::open(path).unwrap();
        let mut reader = BufReader::new(&f);
        let elf_header = ElfHeader::new(&mut reader).unwrap();
        let bin = fs::read(path).unwrap();
        let obj = object::File::parse(&*bin).unwrap();

        // Parsing the Sections
        reader
            .seek(std::io::SeekFrom::Start(elf_header.e_shoff))
            .unwrap();
        let size = elf_header.e_shentsize * elf_header.e_shnum;
        let mut buff = vec![0u8; elf_header.e_shentsize as usize];
        let mut sections: Vec<SectionHeader> = Vec::new();
        for _ in (0..size).step_by(elf_header.e_shentsize as usize) {
            reader.read_exact(&mut buff).unwrap();
            let header = SectionHeader::new(&buff).unwrap();
            sections.push(header);
        }
        for (my, sec) in sections.iter().zip(obj.sections()) {
            assert_eq!(my.sh_size, sec.size());
            assert_eq!(my.sh_addr, sec.address());
            assert_eq!(my.sh_addralign, sec.align());
            // assert_eq!(my,sec.)
        }
    }

    #[test]
    fn relocatable_64() {
        let path = "samples/r64";
        let f = File::open(path).unwrap();
        let bin = fs::read(path).unwrap();
        let obj = object::File::parse(&*bin).unwrap();
        let mut reader = BufReader::new(&f);
        let elf_header = ElfHeader::new(&mut reader).unwrap();
        // Parsing the Sections
        reader
            .seek(std::io::SeekFrom::Start(elf_header.e_shoff))
            .unwrap();

        let size = elf_header.e_shentsize * elf_header.e_shnum;
        let mut buff = vec![0u8; elf_header.e_shentsize as usize];
        let mut sections: Vec<SectionHeader> = Vec::new();
        for _ in (0..size).step_by(elf_header.e_shentsize as usize) {
            reader.read_exact(&mut buff).unwrap();
            let header = SectionHeader::new(&buff).unwrap();
            sections.push(header);
        }

        for (my, sec) in sections.iter().zip(obj.sections()) {
            assert_eq!(my.sh_size, sec.size());
            assert_eq!(my.sh_addr, sec.address());
            assert_eq!(my.sh_addralign, sec.align());
            // assert_eq!(my,sec.)
        }
    }
    #[test]
    fn static_64() {
        let path = "samples/s64";
        let f = File::open(path).unwrap();
        let bin = fs::read(path).unwrap();
        let mut reader = BufReader::new(&f);
        let obj = object::File::parse(&*bin).unwrap();

        let elf_header = ElfHeader::new(&mut reader).unwrap();

        // Parsing the Sections
        reader
            .seek(std::io::SeekFrom::Start(elf_header.e_shoff))
            .unwrap();

        let size = elf_header.e_shentsize * elf_header.e_shnum;
        let mut buff = vec![0u8; elf_header.e_shentsize as usize];
        let mut sections: Vec<SectionHeader> = Vec::new();
        for _ in (0..size).step_by(elf_header.e_shentsize as usize) {
            reader.read_exact(&mut buff).unwrap();
            let header = SectionHeader::new(&buff).unwrap();
            sections.push(header);
        }
        for (my, sec) in sections.iter().zip(obj.sections()) {
            assert_eq!(my.sh_size, sec.size());
            assert_eq!(my.sh_addr, sec.address());
            assert_eq!(my.sh_addralign, sec.align());
            // assert_eq!(my,sec.)
        }
    }
}
