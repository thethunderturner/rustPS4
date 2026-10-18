use crate::helpers::{read_bytes, read_u16, read_u32, read_u64};
use std::io::{self, Read};

#[derive(Debug)]
pub struct SelfHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub mode: u8,
    pub endian: u8,
    pub attributes: u8,
    pub category: u8,
    pub program_type: u8,
    pub reserved1: u16,
    pub header_size: u16,
    pub meta_size: u16,
    pub file_size: u32,
    pub reserved2: u32,
    pub segment_count: u16,
    pub unknown: u16,
    pub reserved3: u32,
}

impl SelfHeader {
    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let magic = read_bytes::<4>(reader)?;
        let version = read_bytes::<1>(reader)?[0];
        let mode = read_bytes::<1>(reader)?[0];
        let endian = read_bytes::<1>(reader)?[0];
        let attributes = read_bytes::<1>(reader)?[0];

        Ok(Self {
            magic,
            version,
            mode,
            endian,
            attributes,
            category: read_bytes::<1>(reader)?[0],
            program_type: read_bytes::<1>(reader)?[0],
            reserved1: read_u16(reader)?,
            header_size: read_u16(reader)?,
            meta_size: read_u16(reader)?,
            file_size: read_u32(reader)?,
            reserved2: read_u32(reader)?,
            segment_count: read_u16(reader)?,
            unknown: read_u16(reader)?,
            reserved3: read_u32(reader)?,
        })
    }
}
