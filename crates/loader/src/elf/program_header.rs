use crate::elf::enums::p_type::PType;
use crate::helpers::{read_u32, read_u64};
use std::io::{self, Read};

#[derive(Debug)]
pub struct ElfProgramHeader {
    pub p_type: PType,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl ElfProgramHeader {
    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            p_type: PType::from_raw(&read_u32(reader)?),
            p_flags: read_u32(reader)?,
            p_offset: read_u64(reader)?,
            p_vaddr: read_u64(reader)?,
            p_paddr: read_u64(reader)?,
            p_filesz: read_u64(reader)?,
            p_memsz: read_u64(reader)?,
            p_align: read_u64(reader)?,
        })
    }
}
