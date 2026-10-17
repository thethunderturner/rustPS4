pub enum EType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    SceExec,
    SceReplayExec,
    SceDynexec,
    SceDynamic,
    Hios,
    Loproc,
    Hiproc,
}

impl EType {
    fn from_str(s: &i32) -> Option<EType> {
        match s {
            0x00 => Some(EType::None),
            0x01 => Some(EType::Rel),
            0x02 => Some(EType::Exec),
            0x03 => Some(EType::Dyn),
            0x04 => Some(EType::Core),
            0xfe00 => Some(EType::SceExec),
            0xfe01 => Some(EType::SceReplayExec),
            0xfe10 => Some(EType::SceDynexec),
            0xfe18 => Some(EType::SceDynamic),
            0xfeff => Some(EType::Hios),
            0xff00 => Some(EType::Loproc),
            0xffff => Some(EType::Hiproc),
            _ => None,
        }
    }
}
