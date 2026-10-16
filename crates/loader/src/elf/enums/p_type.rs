#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PType {
    PtNull,
    PtLoad,
    PtDynamic,
    PtInterp,
    PtNote,
    PtShlib,
    PtPhdr,
    PtTls,
    PtNum,
    PtLoos,
    PtHios,
    PtLoproc,
    PtHiproc,
    Other(u32),
}

impl PType {
    pub fn from_raw(s: &u32) -> Self {
        match s {
            0x00000000 => PType::PtNull,
            0x00000001 => PType::PtLoad,
            0x00000002 => PType::PtDynamic,
            0x00000003 => PType::PtInterp,
            0x00000004 => PType::PtNote,
            0x00000005 => PType::PtShlib,
            0x00000006 => PType::PtPhdr,
            0x00000007 => PType::PtTls,
            0x00000008 => PType::PtNum,
            0x60000000 => PType::PtLoos,
            0x6FFFFFFF => PType::PtHios,
            0x70000000 => PType::PtLoproc,
            0x7FFFFFFF => PType::PtHiproc,
            other => Self::Other(*other),
        }
    }
}