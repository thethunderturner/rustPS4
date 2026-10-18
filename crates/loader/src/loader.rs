use crate::elf::enums::p_type::PType;
use crate::elf::header::ElfHeader;
use crate::elf::program_header::ElfProgramHeader;
use crate::self_format::header::SelfHeader;

use memory::address_space::AddressSpace;
use memory::permissions::Permissions;

use crate::self_format::segment::{SelfSegment, copy_segment, segment_file_offset};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub struct LoadedExecutable {
    pub memory: AddressSpace,
    pub entry_point: u64,
}

pub const BUFFER_SIZE: usize = 64 * 1024;

const LOAD_BIAS: u64 = 0x100000000;

pub fn load(mut f: File) -> io::Result<LoadedExecutable> {
    // Identify the file format.
    let mut magic = [0u8; 4];
    f.read_exact(&mut magic)?;

    // Return to the beginning of the file.
    f.seek(SeekFrom::Start(0))?;

    match magic {
        [0x4F, 0x15, 0x3D, 0x1D] => {
            println!("Detected PS4 SELF");
            load_self(f)
        }
        [0x7F, 0x45, 0x4C, 0x46] => {
            println!("Detected ELF");
            load_elf(f, None)
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unknown executable format",
        )),
    }
}

fn load_self(mut f: File) -> io::Result<LoadedExecutable> {
    let header = SelfHeader::read(&mut f)?;
    let mut segments: Vec<SelfSegment> = Vec::new();

    for _ in 0..header.segment_count {
        segments.push(SelfSegment::read(&mut f)?);
    }

    println!("SELF segments: {}", segments.len());
    load_elf(f, Some(&segments))
}

fn load_elf(mut file: File, self_segments: Option<&[SelfSegment]>) -> io::Result<LoadedExecutable> {
    let elf_base = file.stream_position()?;
    let elf = ElfHeader::read(&mut file)?;

    // ELF program header offsets are relative to the embedded ELF
    let ph_offset = elf_base
        .checked_add(elf.e_phoff)
        .ok_or_else(|| io::Error::other("Program header offset overflow"))?;

    file.seek(SeekFrom::Start(ph_offset))?;

    let mut headers = Vec::with_capacity(elf.e_phnum as usize);

    for _ in 0..elf.e_phnum {
        headers.push(ElfProgramHeader::read(&mut file)?);
    }

    println!("ELF program headers: {}", headers.len());

    let mut space = AddressSpace::new();

    for (index, ph) in headers.iter().enumerate() {
        if ph.p_type != PType::PtLoad {
            continue;
        }

        if ph.p_filesz > ph.p_memsz {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ELF file size exceeds segment memory size",
            ));
        }

        if ph.p_memsz == 0 {
            continue;
        }

        let guest_address = LOAD_BIAS
            .checked_add(ph.p_vaddr)
            .ok_or_else(|| io::Error::other("Guest address overflow"))?;

        let memory_size =
            usize::try_from(ph.p_memsz).map_err(|_| io::Error::other("Segment size too large"))?;

        let permissions = Permissions::new(ph.p_flags);

        space
            .map(guest_address, memory_size, permissions)
            .map_err(io::Error::other)?;

        if ph.p_filesz > 0 {
            let file_offset = segment_file_offset(index, ph, self_segments)?;
            copy_segment(
                &mut file,
                &mut space,
                file_offset,
                guest_address,
                ph.p_filesz,
            )?;
        }

        println!(
            "Loaded PT_LOAD {index}: base={guest_address:#X}, \
             file_size={:#X}, memory_size={:#X}, flags={:#X}",
            ph.p_filesz, ph.p_memsz, ph.p_flags,
        );
    }

    let entry_point = LOAD_BIAS
        .checked_add(elf.e_entry)
        .ok_or_else(|| io::Error::other("Entry point overflow"))?;

    println!("Entry point: {entry_point:#X}");

    if let Ok(byte) = space.read_byte(entry_point) {
        println!("Entry point byte: {byte:#X}");
    }

    Ok(LoadedExecutable {
        memory: space,
        entry_point,
    })
}
