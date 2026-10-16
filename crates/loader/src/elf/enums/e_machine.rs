pub enum EMachine {
    None,
    SPARC,
    X86,
    MIPS,
    PowerPC,
    S390,
    ARM,
    SuperH,
    IA64,
    X86_64,
    AARCH64,
    RiscV,
}

impl EMachine {
    fn from_str(s: &i32) -> Option<EMachine> {
        match s {
            0x00 => Some(EMachine::None),
            0x02 => Some(EMachine::SPARC),
            0x03 => Some(EMachine::X86),
            0x08 => Some(EMachine::MIPS),
            0x14 => Some(EMachine::PowerPC),
            0x16 => Some(EMachine::S390),
            0x28 => Some(EMachine::ARM),
            0x2A => Some(EMachine::SuperH),
            0x32 => Some(EMachine::IA64),
            0x3E => Some(EMachine::X86_64),
            0xB7 => Some(EMachine::AARCH64),
            0xF3 => Some(EMachine::RiscV),
            _ => None,
        }
    }
}
