use std::io;
use std::io::Read;

pub fn read_bytes<const N: usize>(reader: &mut impl Read) -> io::Result<[u8; N]> {
    let mut buffer = [0u8; N];
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}

pub fn read_u16(reader: &mut impl Read) -> io::Result<u16> {
    let bytes = read_bytes::<2>(reader)?;
    Ok(u16::from_le_bytes(bytes))
}

pub fn read_u32(reader: &mut impl Read) -> io::Result<u32> {
    let bytes = read_bytes::<4>(reader)?;
    Ok(u32::from_le_bytes(bytes))
}
pub fn read_u64(reader: &mut impl Read) -> io::Result<u64> {
    let bytes = read_bytes::<8>(reader)?;
    Ok(u64::from_le_bytes(bytes))
}
