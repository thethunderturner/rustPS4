use std::fs::File;
use std::io;
use loader::loader::load;

fn main() -> io::Result<()> {
    let f = File::open("/run/media/mateo/SEAGATE4TB/ps4/Games/CUSA02368/eboot.bin")?;
    load(f).expect("TODO: panic message");

    Ok(())
}
