use crate::helpers::{read_bytes, read_u64};
use std::io::{self, Read};

pub const DT_NULL: i64 = 0;

#[derive(Debug)]
pub struct DynamicEntry {
    pub tag: i64,
    pub value: u64,
}

pub fn read_dynamic(reader: &mut impl Read, byte_size: u64) -> io::Result<Vec<DynamicEntry>> {
    const ENTRY_SIZE: u64 = 16;

    if byte_size % ENTRY_SIZE != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Dynamic table size is not a multiple of 16",
        ));
    }

    let mut entries = Vec::new();

    for _ in 0..byte_size / ENTRY_SIZE {
        let tag = i64::from_le_bytes(read_bytes::<8>(reader)?);
        let value = read_u64(reader)?;

        if tag == DT_NULL {
            return Ok(entries);
        }

        entries.push(DynamicEntry { tag, value });
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Dynamic table is missing DT_NULL",
    ))
}
