use std::fmt;

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use num_traits::PrimInt;

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

pub struct HexDisplayRef32<'a>(pub &'a [u32]);

impl fmt::Display for HexDisplayRef32<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.0 {
            // we need to swap here to *display* LE order
            write!(f, "{:08x}", x.swap_bytes())?;
        }

        Ok(())
    }
}

impl fmt::Debug for HexDisplayRef32<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

pub fn read_u32(buf: &[u8]) -> u32 {
    if cfg!(target_endian = "big") {
        BigEndian::read_u32(buf)
    } else {
        LittleEndian::read_u32(buf)
    }
}

pub(crate) fn into_bytes<T: bytemuck::Pod + PrimInt>(
    mut checksum: [T; 1024],
) -> Vec<u8> {
    // pessimization for big endian platforms, byte swapping is required because the words are currently in big endian order and need to be reversed.
    if cfg!(target_endian = "big") {
        for elem in &mut checksum {
            *elem = elem.swap_bytes();
        }
    }

    bytemuck::bytes_of(&checksum).to_vec()
}
