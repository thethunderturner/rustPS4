use std::io::{self, Read};
use crate::helpers::{read_bytes, read_u16, read_u32, read_u64};

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

#[derive(Debug)]
pub struct SelfSegment {
    pub flags: u64,
    pub file_offset: u64,
    pub file_size: u64,
    pub memory_size: u64,
}

impl SelfSegment {
    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            flags: read_u64(reader)?,
            file_offset: read_u64(reader)?,
            file_size: read_u64(reader)?,
            memory_size: read_u64(reader)?,
        })
    }
    pub fn segment_id(&self) -> u64 { self.flags >> 20 }
    pub fn is_ordered(&self) -> bool {
        self.flags & 0x1 != 0
    }
    pub fn is_encrypted(&self) -> bool { self.flags & 0x2 != 0 }
    pub fn is_signed(&self) -> bool {
        self.flags & 0x4 != 0
    }
    pub fn is_compressed(&self) -> bool {
        self.flags & 0x8 != 0
    }
    pub fn is_blocked(&self) -> bool { self.flags & 0x800 != 0 }
}