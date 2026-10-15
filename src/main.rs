use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut f = File::open("/run/media/mateo/ExtremeMainPart/Games/Emulation/dumped_games/ps4/Hidden/CUSA12606/eboot.bin")?;
    let mut buffer = [0; 8];

    // read up to 8 bytes
    f.read_exact(&mut buffer)?;
    for byte in buffer {
        print!("{byte:02X} ");
    }
    println!();

    let magic = u32::from_le_bytes([
        buffer[0],
        buffer[1],
        buffer[2],
        buffer[3],
    ]);

    println!("Magic: {magic:#010X}");

    Ok(())
}
