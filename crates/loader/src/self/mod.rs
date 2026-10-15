pub struct SelfHeader {
    magic: [u8; 4], // 4F 15 3D 1D
    unknown1: [u8; 4], // Always 00 01 01 12
    program_type: [u8; 4],
    header_size: [u8; 2],
    signature: [u8; 2],
    file_size: [u8; 4],
    padding1: [u8; 4],
    nr_of_segments: [u8; 2],
    unknown2: [u8; 2],
    padding2: [u8; 4],
}

pub struct SelfSegment {
    pub flags: u64,
    pub file_offset: u64,
    pub file_size: u64,
    pub memory_size: u64,
}