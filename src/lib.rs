mod lthash;

use byteorder::{BigEndian, ByteOrder, LittleEndian};
pub use lthash::*;

fn read_u16(buf: &[u8]) -> u16 {
    if cfg!(target_endian = "big") {
        BigEndian::read_u16(buf)
    } else {
        LittleEndian::read_u16(buf)
    }
}
