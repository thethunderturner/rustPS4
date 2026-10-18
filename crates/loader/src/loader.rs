
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

use crate::elf::header::ElfHeader;
use crate::elf::program_header::ElfProgramHeader;
use crate::self_format::header::{SelfHeader, SelfSegment};

pub fn load(mut f: File) -> io::Result<()> {
    // Identify the file format.
    let mut magic = [0u8; 4];
    f.read_exact(&mut magic)?;

    // Return to the beginning of the file.
    f.seek(SeekFrom::Start(0))?;

    match magic {
        [0x4F, 0x15, 0x3D, 0x1D] => {
            println!("Detected PS4 SELF");
            load_self(f)?
        }
        [0x7F, 0x45, 0x4C, 0x46] => {
            println!("Detected ELF");
            load_elf(f)?
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown executable format",
            ));
        }
    }

    Ok(())
}

fn load_self(mut f: File) -> io::Result<()> {
    let header = SelfHeader::read(&mut f)?;
    let mut segments: Vec<SelfSegment> = Vec::new();

    for _ in 0..header.segment_count {
        segments.push(SelfSegment::read(&mut f)?);
    }
    load_elf(f)?;

    Ok(())
}

fn load_elf(mut f: File) -> io::Result<()> {
    // Save where the ELF begins in the containing file.
    let elf_base = f.stream_position()?;

    let elf = ElfHeader::read(&mut f)?;
    let mut headers: Vec<ElfProgramHeader> = Vec::new();

    // e_phoff is relative to the beginning of the ELF.
    let ph_offset = elf_base
        .checked_add(elf.e_phoff)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "ELF program header offset overflow",
            )
        })?;

    f.seek(SeekFrom::Start(ph_offset))?;

    for _ in 0..elf.e_phnum {
        headers.push(ElfProgramHeader::read(&mut f)?);
    }

    println!("ELF program headers: {}", headers.len());

    for ph in &headers {
        println!("{ph:#?}");
    }

    Ok(())
}
