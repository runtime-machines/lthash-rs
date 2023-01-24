use std::marker::PhantomData;

use byteorder::{ByteOrder, LittleEndian};
use digest::ExtendableOutput;

pub trait LtHash {
    fn insert(&mut self, element: impl AsRef<[u8]>);
    fn remove(&mut self, element: impl AsRef<[u8]>);
    fn checksum(&self) -> Vec<u8>;
}

pub struct LtHash16<T: ExtendableOutput + Default> {
    checksum: [u8; 2048],
    hasher: PhantomData<T>,
}

impl<T> LtHash16<T>
where
    T: ExtendableOutput + Default,
{
    pub fn as_bytes(&self) -> &[u8; 2048] {
        &self.checksum
    }

    fn hash_object(&mut self, object: impl AsRef<[u8]>) -> [u8; 2048] {
        let mut output = [0u8; 2048];
        T::digest_xof(object, output.as_mut());
        output
    }
}

impl<T> LtHash for LtHash16<T>
where
    T: ExtendableOutput + Default,
{
    fn insert(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        let mut i = 0;
        while i < 2048 {
            let xi = &hashed[i..i + 2];
            let yi = &self.checksum[i..i + 2];
            let xi = LittleEndian::read_u16(xi);
            let yi = LittleEndian::read_u16(yi);
            let sum = xi.wrapping_add(yi);
            LittleEndian::write_u16(&mut self.checksum[i..i + 2], sum);
            i += 2;
        }
    }

    fn remove(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        let mut i = 0;
        while i < 2048 {
            let xi = &hashed[i..i + 2];
            let yi = &self.checksum[i..i + 2];
            let xi = LittleEndian::read_u16(xi);
            let yi = LittleEndian::read_u16(yi);
            let diff = xi.wrapping_sub(yi);
            LittleEndian::write_u16(&mut self.checksum[i..i + 2], diff);
            i += 2;
        }
    }

    fn checksum(&self) -> Vec<u8> {
        self.checksum.to_vec()
    }
}
