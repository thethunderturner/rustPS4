enum EIdent {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    GnuHurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOS,
    CloudABI
}

impl EIdent {
    fn from_str(s: &i32) -> Option<EIdent> {
        match s {
            0x00 => Some(EIdent::SystemV),
            0x01 => Some(EIdent::HPUX),
            0x02 => Some(EIdent::NetBSD),
            0x03 => Some(EIdent::Linux),
            0x04 => Some(EIdent::GnuHurd),
            0x06 => Some(EIdent::Solaris),
            0x07 => Some(EIdent::AIX),
            0x08 => Some(EIdent::IRIX),
            0x09 => Some(EIdent::FreeBSD),
            0x0A => Some(EIdent::Tru64),
            0x0B => Some(EIdent::NovellModesto),
            0x0C => Some(EIdent::OpenBSD),
            0x0D => Some(EIdent::OpenVMS),
            0x0E => Some(EIdent::NonStopKernel),
            0x0F => Some(EIdent::AROS),
            0x10 => Some(EIdent::FenixOS),
            0x11 => Some(EIdent::CloudABI),
            _ => None,
        }
    }
}