use std::io::{self, Read};

use crate::helpers::{read_bytes, read_u16, read_u32, read_u64};

#[derive(Debug)]
pub struct Elf {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub osabi: u8,
    pub abiversion: u8,
    pub padding: [u8; 7],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
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

impl Elf {
    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let magic = read_bytes::<4>(reader)?;

        if magic != [0x7F, 0x45, 0x4C, 0x46] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid ELF magic",
            ));
        }

        let class = read_bytes::<1>(reader)?[0];
        let data = read_bytes::<1>(reader)?[0];

        if class != 2 || data != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected little-endian ELF64",
            ));
        }

        Ok(Self {
            magic,
            class,
            data,
            version: read_bytes::<1>(reader)?[0],
            osabi: read_bytes::<1>(reader)?[0],
            abiversion: read_bytes::<1>(reader)?[0],
            padding: read_bytes::<7>(reader)?,
            e_type: read_u16(reader)?,
            e_machine: read_u16(reader)?,
            e_version: read_u32(reader)?,
            e_entry: read_u64(reader)?,
            e_phoff: read_u64(reader)?,
            e_shoff: read_u64(reader)?,
            e_flags: read_u32(reader)?,
            e_ehsize: read_u16(reader)?,
            e_phentsize: read_u16(reader)?,
            e_phnum: read_u16(reader)?,
            e_shentsize: read_u16(reader)?,
            e_shnum: read_u16(reader)?,
            e_shstrndx: read_u16(reader)?,
        })
    }
}