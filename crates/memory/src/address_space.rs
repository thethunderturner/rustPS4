use crate::guest::{GuestMemory, overlaps};

#[derive(Debug)]
pub struct AddressSpace {
    regions: Vec<GuestMemory>,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    pub fn map(&mut self, base: u64, size: usize) -> Result<(), &'static str> {
        // Reject regions size 0
        if size == 0 {
            return Err("Region size cannot be 0");
        }

        // Check whether base+size overflows
        let size_u64 = u64::try_from(size).map_err(|_| "Region size too large")?;

        // Check whether base + size overflows.
        base.checked_add(size_u64)
            .ok_or("Region address overflow")?;

        for region in &self.regions {
            // chceck for overlaps
            let region_size =
                u64::try_from(region.bytes.len()).map_err(|_| "Existing region size too large")?;

            if overlaps(base, size_u64, region.base, region_size)
                .ok_or("Region address overflow")?
            {
                return Err("Region overlaps with existing region");
            }
        }

        self.regions.push(GuestMemory::new(base, size));

        Ok(())
    }

    // Given an address, search through the regions in virtual memory and return the byte that matches the address
    pub fn read_byte(&self, address: u64) -> Option<u8> {
        for region in &self.regions {
            if let Some(byte) = region.read_byte(address) {
                return Some(byte);
            }
        }

        None
    }

    // Given an address and a byte, it searches through the regions in virtual memory and writes the byte to the address
    pub fn write_byte(&mut self, address: u64, value: u8) -> Option<()> {
        for region in &mut self.regions {
            if region.write_byte(address, value).is_some() {
                return Some(());
            }
        }

        None
    }
}
