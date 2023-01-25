use std::fmt;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub struct HexDisplayRef16<'a>(pub &'a [u16]);

impl fmt::Display for HexDisplayRef16<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.0 {
            // we need to swap here to *display* LE order
            write!(f, "{:04x}", x.swap_bytes())?;
        }

        Ok(())
    }
}

impl fmt::Debug for HexDisplayRef16<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

pub fn read_u16(buf: &[u8]) -> u16 {
    if cfg!(target_endian = "big") {
        BigEndian::read_u16(buf)
    } else {
        LittleEndian::read_u16(buf)
    }
}
