use crate::elf::enums::p_type::PType;
use crate::elf::header::ElfHeader;
use crate::elf::program_header::ElfProgramHeader;
use crate::self_format::header::{SelfHeader, SelfSegment};
use memory::address_space::AddressSpace;
use memory::permissions::Permissions;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

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
    let ph_offset = elf_base.checked_add(elf.e_phoff).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "ELF program header offset overflow",
        )
    })?;

    f.seek(SeekFrom::Start(ph_offset))?;

    for _ in 0..elf.e_phnum {
        headers.push(ElfProgramHeader::read(&mut f)?);
    }

    let load_bias: u64 = 0x100000000;
    let mut space = AddressSpace::new();

    for ph in &headers {
        if ph.p_type != PType::PtLoad {
            continue;
        }

        if ph.p_filesz > ph.p_memsz {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ELF segment file size exceeds memory size",
            ));
        }

        let addr = load_bias
            .checked_add(ph.p_vaddr)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Guest address overflow"))?;

        let size = usize::try_from(ph.p_memsz)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Segment size too large"))?;
        let perms = Permissions::new(ph.p_flags);
        space
            .map(addr, size, perms)
            .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;

        f.seek(SeekFrom::Start(ph.p_offset))?;

        const BUFFER_SIZE: usize = 64 * 1024; // Buffer size of 64KiB (for now at leaast)

        let mut buffer = [0u8; BUFFER_SIZE];
        let mut remaining = ph.p_filesz;
        let mut guest_address = addr;

        while remaining > 0 {
            let bytes_to_read = remaining.min(BUFFER_SIZE as u64);
            let bytes_to_read = usize::try_from(bytes_to_read)
                .map_err(|_| io::Error::other("Read size too large"))?;

            f.read_exact(&mut buffer[..bytes_to_read])?;

            space
                .load_bytes(guest_address, &buffer[..bytes_to_read])
                .map_err(io::Error::other)?;

            // Advance the guest address
            guest_address = guest_address
                .checked_add(bytes_to_read as u64)
                .ok_or_else(|| io::Error::other("Guest address overflow"))?;

            // Reduce the number of remaining bytes
            remaining -= bytes_to_read as u64;
        }

        println!(
            "Loaded PT_LOAD: base={addr:#X}, file_size={:#X}, memory_size={:#X}",
            ph.p_filesz,
            ph.p_memsz,
        );
    }

    let entry_address = load_bias
        .checked_add(elf.e_entry)
        .ok_or_else(|| io::Error::other("Entry point overflow"))?;
    println!(
        "Entry point byte: {:#X}",
        space.read_byte(entry_address).map_err(io::Error::other)?
    );
    Ok(())
}
