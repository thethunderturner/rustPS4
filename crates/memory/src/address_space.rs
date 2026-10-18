use crate::guest::{GuestMemory, overlaps};
use crate::permissions::Permissions;

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

    pub fn map(
        &mut self,
        base: u64,
        size: usize,
        permissions: Permissions,
    ) -> Result<(), &'static str> {
        // Reject regions size 0
        if size == 0 {
            return Err("Region size cannot be 0");
        }

        // Check whether base+size overflows
        let size_u64 = u64::try_from(size).map_err(|_| "Region size too large")?;
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

        self.regions.push(GuestMemory::new(base, size, permissions));

        Ok(())
    }

    // Given an address, search through the regions in virtual memory and return the byte that matches the address
    pub fn read_byte(&self, address: u64) -> Result<u8, &'static str> {
        for region in &self.regions {
            if region.contains(address) {
                if !region.permissions.can_read() {
                    return Err("Tried to read in region without read permission");
                };
                if let Some(byte) = region.read_byte(address) {
                    return Ok(byte);
                }
            }
        }

        Err("Address is not mapped")
    }

    // Given an address and a byte, it searches through the regions in virtual memory and writes the byte to the address
    pub fn write_byte(&mut self, address: u64, value: u8) -> Result<(), &'static str> {
        for region in &mut self.regions {
            if region.contains(address) {
                if !region.permissions.can_write() {
                    return Err("Tried to write in region without write permission");
                };
                if region.write_byte(address, value).is_some() {
                    return Ok(());
                } else {
                    return Err("Failed to write byte to address");
                }
            }
        }

        Err("Address is not mapped")
    }

    // Given an address and a byte, it searches through the regions in virtual memory and writes the byte to the address. This function does not check for write permissions.
    // Its only for the first time, when we read the bytes from the elf, and we want to put it in memory. After that we need permissions to r/w/e
    pub fn load_byte(&mut self, address: u64, value: u8) -> Result<(), &'static str> {
        for region in &mut self.regions {
            if region.contains(address) {
                if region.write_byte(address, value).is_some() {
                    return Ok(());
                } else {
                    return Err("Failed to write byte to address");
                }
            }
        }
        Err("Address is not mapped")
    }

    pub fn load_bytes(&mut self, address: u64, data: &[u8]) -> Result<(), &'static str> {
        for (index, byte) in data.iter().enumerate() {
            let index = u64::try_from(index).map_err(|_| "Index too large")?;
            let next_address = address.checked_add(index).ok_or("Address overflow")?;
            self.load_byte(next_address, *byte)?;
        }

        Ok(())
    }
}
