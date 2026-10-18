use loader::loader::load;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let f = File::open(
        "/run/media/mateo/ExtremeMainPart/Games/Emulation/dumped_games/ps4/Hidden/CUSA57548/eboot.bin",
    )?;
    load(f)?;

    Ok(())
}
