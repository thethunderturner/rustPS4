#[derive(Debug)]
struct GuestMemory {
    base: u64,
    bytes: Vec<u8>,
}

impl GuestMemory {
    fn new(base: u64, size: usize) -> Self {
        Self {
            base,
            bytes: vec![0; size],
        }
    }

    fn read_byte(&self, address: u64) -> Option<u8> {
        let offset = address.checked_sub(self.base)?;
        let index = usize::try_from(offset).ok()?;

        self.bytes.get(index).copied()
    }

    fn write_byte(&mut self, address: u64, value: u8) -> Option<()> {
        let offset = address.checked_sub(self.base)?;
        let index = usize::try_from(offset).ok()?;

        let byte = self.bytes.get_mut(index)?;
        *byte = value;

        Some(())
    }

}

fn main() {
    let file = [
        0xAA, 0xBB, 0xCC, 0xDD,
        0x11, 0x22, 0x33, 0x44,
    ];

    let mut memory = GuestMemory::new(0x500000, 8);

    load_segment(
        &mut memory,
        &file,
        4, // File offset
        0x500000, // Virtual address
        4, // p_filesz
        8, // p_memsz
    ).unwrap();

    println!("{:02X?}", memory.bytes);
}
fn load_segment(
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
        let file_index = file_offset
            .checked_add(i)
            .ok_or("File offset overflow")?;
        let byte = *file
            .get(file_index)
            .ok_or("File offset is out of bounds")?;
        let address = virtual_address
            .checked_add(u64::try_from(i).map_err(|_| "Offset too large")?)
            .ok_or("Virtual address overflow")?;
        memory
            .write_byte(address, byte)
            .ok_or("Guest memory address is invalid")?;
    }

    Ok(())
}