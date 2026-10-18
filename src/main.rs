use loader::elf::enums::p_type::PType;
use loader::elf::header::ElfHeader;
use loader::elf::program_header::ElfProgramHeader;
use loader::self_format::header::{SelfHeader, SelfSegment};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

fn main() -> io::Result<()> {
    let mut f = File::open("/run/media/mateo/SEAGATE4TB/ps4/Games/CUSA02368/eboot.bin")?;

    // Identify the file format
    let mut magic = [0u8; 4];
    f.read_exact(&mut magic)?;

    // Return to the beginning of the file.
    f.seek(SeekFrom::Start(0))?;

    match magic {
        [0x4F, 0x15, 0x3D, 0x1D] => {
            println!("Detected PS4 SELF");

            let header = SelfHeader::read(&mut f)?;
            let mut segments: Vec<SelfSegment> = Vec::new();
            let mut headers: Vec<ElfProgramHeader> = Vec::new();

            for _i in 0..header.segment_count {
                segments.push(SelfSegment::read(&mut f)?);
            }

            let elf = ElfHeader::read(&mut f)?;
            for _i in 0..elf.e_phnum {
                headers.push(ElfProgramHeader::read(&mut f)?);
            }

            println!("{:#?}", segments);
            println!("{:#?}", headers);
        }

        [0x7F, 0x45, 0x4C, 0x46] => {
            println!("Detected ELF");
            let mut headers: Vec<ElfProgramHeader> = Vec::new();
            let elf = ElfHeader::read(&mut f)?;
            for _i in 0..elf.e_phnum {
                headers.push(ElfProgramHeader::read(&mut f)?);
            }
            // println!("{:#?}", headers);
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
