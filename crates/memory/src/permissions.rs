#[derive(Debug, Clone, Copy)]
pub struct Permissions {
    bits: u32,
}

impl Permissions {
    pub const READ: u32 = 0x4;
    pub const WRITE: u32 = 0x2;
    pub const EXECUTE: u32 = 0x1;

    pub fn new(bits: u32) -> Self {
        Self { bits }
    }

    pub fn can_read(&self) -> bool {
        (self.bits & Self::READ) != 0
    }

    pub fn can_write(&self) -> bool {
        (self.bits & Self::WRITE) != 0
    }

    pub fn can_execute(&self) -> bool {
        (self.bits & Self::EXECUTE) != 0
    }
}