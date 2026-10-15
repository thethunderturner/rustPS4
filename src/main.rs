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
    let mut memory = GuestMemory::new(0x400000, 4);
    println!("{:#?}", memory);

    memory.write_byte(0x400000, 0xAA).unwrap();
    memory.write_byte(0x400001, 0xBB).unwrap();
    memory.write_byte(0x400002, 0xCC).unwrap();
    memory.write_byte(0x400003, 0xFF).unwrap();
    memory.write_byte(0x400004, 0xDD).unwrap();

    println!("{:02X}", memory.read_byte(0x400000).unwrap());
    println!("{:02X}", memory.read_byte(0x400001).unwrap());
    println!("{:02X}", memory.read_byte(0x400002).unwrap());
    println!("{:02X}", memory.read_byte(0x400003).unwrap());
}