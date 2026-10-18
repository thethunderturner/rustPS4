use crate::elf::program_header::ElfProgramHeader;
use crate::helpers::read_u64;
use memory::address_space::AddressSpace;
use memory::guest::GuestMemory;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

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
    pub fn id(&self) -> u64 {
        self.flags >> 20
    }
    pub fn is_ordered(&self) -> bool {
        self.flags & 0x1 != 0
    }
    pub fn is_encrypted(&self) -> bool {
        self.flags & 0x2 != 0
    }
    pub fn is_signed(&self) -> bool {
        self.flags & 0x4 != 0
    }
    pub fn is_compressed(&self) -> bool {
        self.flags & 0x8 != 0
    }
    pub fn is_blocked(&self) -> bool {
        self.flags & 0x800 != 0
    }
}

pub fn load_segment(
    memory: &mut GuestMemory,
    file: &[u8],
    file_offset: usize,
    virtual_address: u64,
    file_size: usize,
    memory_size: usize,
) -> Result<(), &'static str> {
    if memory_size < file_size {
        return Err("Memory size is smaller than file size");
    }

    for i in 0..file_size {
        let file_index = file_offset.checked_add(i).ok_or("File offset overflow")?;
        let byte = *file.get(file_index).ok_or("File offset is out of bounds")?;
        let address = virtual_address
            .checked_add(u64::try_from(i).map_err(|_| "Offset too large")?)
            .ok_or("Virtual address overflow")?;
        memory
            .write_byte(address, byte)
            .ok_or("Guest memory address is invalid")?;
    }

    Ok(())
}

pub fn copy_segment(
    file: &mut File,
    space: &mut AddressSpace,
    file_offset: u64,
    guest_address: u64,
    file_size: u64,
) -> io::Result<()> {
    file.seek(SeekFrom::Start(file_offset))?;

    let mut buffer = [0u8; crate::loader::BUFFER_SIZE];
    let mut remaining = file_size;
    let mut destination = guest_address;

    while remaining > 0 {
        let chunk_size = remaining.min(crate::loader::BUFFER_SIZE as u64);
        let chunk_size =
            usize::try_from(chunk_size).map_err(|_| io::Error::other("Chunk size too large"))?;

        file.read_exact(&mut buffer[..chunk_size])?;
        space
            .load_bytes(destination, &buffer[..chunk_size])
            .map_err(io::Error::other)?;
        destination = destination
            .checked_add(chunk_size as u64)
            .ok_or_else(|| io::Error::other("Guest address overflow"))?;
        remaining -= chunk_size as u64;
    }

    Ok(())
}

pub fn segment_file_offset(
    index: usize,
    ph: &ElfProgramHeader,
    self_segments: Option<&[SelfSegment]>,
) -> io::Result<u64> {
    let Some(segments) = self_segments else {
        return Ok(ph.p_offset);
    };

    let ph_id =
        u64::try_from(index).map_err(|_| io::Error::other("Program header index too large"))?;
    let segment = segments
        .iter()
        .find(|segment| segment.is_blocked() && segment.id() == ph_id)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No matching SELF segment"))?;
    if segment.is_encrypted() || segment.is_compressed() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Encrypted or compressed SELF segments are not supported",
        ));
    }

    if segment.file_size != ph.p_filesz || segment.memory_size != ph.p_filesz {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "SELF segment requires additional data processing",
        ));
    }

    println!(
        "SELF PHDR {index}: file_offset={:#X}, size={:#X}",
        segment.file_offset, segment.file_size,
    );

    Ok(segment.file_offset)
}

// finds where the requested ELF data is physically stored
pub fn elf_range_file_offset(
    index: usize,
    headers: &[ElfProgramHeader],
    self_segments: Option<&[SelfSegment]>,
) -> io::Result<u64> {
    let invalid = || io::Error::new(io::ErrorKind::InvalidData, "Invalid ELF segment range");
    let ph = headers.get(index).ok_or_else(invalid)?;
    ph.p_offset.checked_add(ph.p_filesz).ok_or_else(invalid)?;
    let Some(segments) = self_segments else {
        return Ok(ph.p_offset);
    };

    if segments
        .iter()
        .any(|s| s.is_blocked() && s.id() == index as u64)
    {
        return segment_file_offset(index, ph, Some(segments));
    }

    for (container_index, container) in headers.iter().enumerate() {
        let Some(relative) = ph.p_offset.checked_sub(container.p_offset) else {
            continue;
        };
        if relative > container.p_filesz || ph.p_filesz > container.p_filesz - relative {
            continue;
        }
        if !segments
            .iter()
            .any(|s| s.is_blocked() && s.id() == container_index as u64)
        {
            continue;
        }
        let base = segment_file_offset(container_index, container, Some(segments))?;
        let offset = base.checked_add(relative).ok_or_else(invalid)?;
        offset.checked_add(ph.p_filesz).ok_or_else(invalid)?;
        return Ok(offset);
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "No SELF segment contains the requested ELF range",
    ))
}
