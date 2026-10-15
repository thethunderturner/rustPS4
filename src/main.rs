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

    let mut memory = GuestMemory::new(0x500000, 4);

    load_segment(&mut memory, &file, 4, 0x500000, 4).unwrap();

    println!("{:02X?}", memory.bytes);
}

fn load_segment(
    memory: &mut GuestMemory,
    file: &[u8],
    file_offset: usize,
    virtual_address: u64,
    segment_file_size: usize,
) -> Option<()> {
    for i in 0..segment_file_size {
        let file_index = file_offset.checked_add(i)?;
        let byte = *file.get(file_index)?;

        let address = virtual_address.checked_add(i as u64)?;
        memory.write_byte(address, byte)?;
    }

    Some(())
}